# Copyright 2012 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.


# Create variables HOST_<triple> containing the host part
# of each target triple.  For example, the triple i686-darwin-macos
# would create a variable HOST_i686-darwin-macos with the value 
# i386.
define DEF_HOST_VAR
  HOST_$(1) = $(subst i686,i386,$(word 1,$(subst -, ,$(1))))
endef
$(foreach t,$(CFG_TARGET_TRIPLES),$(eval $(call DEF_HOST_VAR,$(t))))
$(foreach t,$(CFG_TARGET_TRIPLES),$(info cfg: host for $(t) is $(HOST_$(t))))

# Ditto for OSTYPE
define DEF_OSTYPE_VAR
  OSTYPE_$(1) = $(subst $(firstword $(subst -, ,$(1)))-,,$(1))
endef
$(foreach t,$(CFG_TARGET_TRIPLES),$(eval $(call DEF_OSTYPE_VAR,$(t))))
$(foreach t,$(CFG_TARGET_TRIPLES),$(info cfg: os for $(t) is $(OSTYPE_$(t))))

# FIXME: no-omit-frame-pointer is just so that task_start_wrapper
# has a frame pointer and the stack walker can understand it. Turning off
# frame pointers everywhere is overkill
CFG_GCCISH_CFLAGS += -fno-omit-frame-pointer

# On Darwin, we need to run dsymutil so the debugging information ends
# up in the right place.  On other platforms, it automatically gets
# embedded into the executable, so use a no-op command.
CFG_DSYMUTIL := true

# Add a dSYM glob for all platforms, even though it will do nothing on
# non-Darwin platforms; omitting it causes a full -R copy of lib/
CFG_LIB_DSYM_GLOB=lib$(1)-*.dylib.dSYM

# Hack: not sure how to test if a file exists in make other than this
OS_SUPP = $(patsubst %,--suppressions=%,\
      $(wildcard $(CFG_SRC_DIR)src/etc/$(CFG_OSTYPE).supp*))

ifdef CFG_DISABLE_OPTIMIZE_CXX
  $(info cfg: disabling C++ optimization (CFG_DISABLE_OPTIMIZE_CXX))
  CFG_GCCISH_CFLAGS += -O0
else
  CFG_GCCISH_CFLAGS += -O2
endif

ifdef CFG_VALGRIND
  CFG_VALGRIND += --error-exitcode=100 \
                  --quiet \
                  --suppressions=$(CFG_SRC_DIR)src/etc/x86.supp \
                  $(OS_SUPP)
  ifdef CFG_ENABLE_HELGRIND
    CFG_VALGRIND += --tool=helgrind
  else
    CFG_VALGRIND += --tool=memcheck \
                    --leak-check=full
  endif
endif

ifneq ($(findstring linux,$(CFG_OSTYPE)),)
  ifdef CFG_PERF
    ifneq ($(CFG_PERF_WITH_LOGFD),)
        CFG_PERF_TOOL := $(CFG_PERF) stat -r 3 --log-fd 2
    else
        CFG_PERF_TOOL := $(CFG_PERF) stat -r 3
    endif
  else
    ifdef CFG_VALGRIND
      CFG_PERF_TOOL :=\
        $(CFG_VALGRIND) --tool=cachegrind --cache-sim=yes --branch-sim=yes
    else
      CFG_PERF_TOOL := /usr/bin/time --verbose
    endif
  endif
endif


# Configure toolchains per target triple


define CFG_MAKE_TOOLCHAIN

ifeq (i386,$$(HOST_$(1)))
  CFG_GCCISH_CFLAGS_$(1) += -m32
  CFG_GCCISH_LINK_FLAGS_$(1) += -m32
endif

ifeq (x86_64,$$(HOST_$(1)))
  CFG_GCCISH_CFLAGS_$(1) += -m64
  CFG_GCCISH_LINK_FLAGS_$(1) += -m64
endif

ifneq ($$(findstring freebsd,$$(OSTYPE_$(1))),)
  CFG_UNIXY_$(1) := 1
  CFG_LIB_NAME_$(1)=lib$$(1).so
  CFG_LIB_GLOB_$(1)=lib$$(1)-*.so
  CFG_GCCISH_CFLAGS_$(1) += -fPIC -I/usr/local/include
  CFG_GCCISH_LINK_FLAGS_$(1) += -shared -fPIC -lpthread -lrt
  CFG_GCCISH_DEF_FLAG_$(1) := -Wl,--export-dynamic,--dynamic-list=
  CFG_GCCISH_PRE_LIB_FLAGS_$(1) := -Wl,-whole-archive
  CFG_GCCISH_POST_LIB_FLAGS_$(1) := -Wl,-no-whole-archive
  CFG_FBSD_$(1) := 1
  CFG_LDENV_$(1) := LD_LIBRARY_PATH
  CFG_DEF_SUFFIX_$(1) := .bsd.def
  CFG_INSTALL_NAME_$(1) =
  CFG_PERF_TOOL_$(1) := /usr/bin/time
  CFG_LIBUV_LINK_FLAGS_$(1) := -lpthread -lkvm
endif

ifneq ($$(findstring linux,$$(OSTYPE_$(1))),)
  CFG_UNIXY_$(1) := 1
  CFG_LIB_NAME_$(1)=lib$$(1).so
  CFG_LIB_GLOB_$(1)=lib$$(1)-*.so
  CFG_GCCISH_CFLAGS_$(1) += -fPIC
  CFG_GCCISH_LINK_FLAGS_$(1) += -shared -fPIC -ldl -lpthread -lrt
  CFG_GCCISH_DEF_FLAG_$(1) := -Wl,--export-dynamic,--dynamic-list=
  CFG_GCCISH_PRE_LIB_FLAGS_$(1) := -Wl,-whole-archive
  # -znoexecstack is here because librt is for some reason being created
  # with executable stack and Fedora (or SELinux) doesn't like that (#798)
  CFG_GCCISH_POST_LIB_FLAGS_$(1) := -Wl,-no-whole-archive -Wl,-znoexecstack
  CFG_LDENV_$(1) := LD_LIBRARY_PATH
  CFG_DEF_SUFFIX_$(1) := .linux.def
  CFG_INSTALL_NAME_$(1) =
  # Linux requires LLVM to be built like this to get backtraces into Rust code
  CFG_LLVM_BUILD_ENV_$(1)="CXXFLAGS=-fno-omit-frame-pointer"
  CFG_LIBUV_LINK_FLAGS_$(1) := -lpthread
endif

ifneq ($$(findstring darwin,$$(OSTYPE_$(1))),)
  CFG_UNIXY_$(1) := 1
  CFG_LIB_NAME_$(1)=lib$$(1).dylib
  CFG_LIB_GLOB_$(1)=lib$$(1)-*.dylib
  CFG_LDENV_$(1) := DYLD_LIBRARY_PATH
  CFG_GCCISH_LINK_FLAGS_$(1) += -dynamiclib -lpthread -framework CoreServices -Wl,-no_compact_unwind
  CFG_GCCISH_DEF_FLAG_$(1) := -Wl,-exported_symbols_list,
  # Darwin has a very blurry notion of "64 bit", and claims it's running
  # "on an i386" when the whole userspace is 64-bit and the compiler
  # emits 64-bit binaries by default. So we just force -m32 here. Smarter
  # approaches welcome!
  #
  # NB: Currently GCC's optimizer breaks rustrt (task-comm-1 hangs) on Darwin.
  ifeq (i386,$$(HOST_$(1)))
    CFG_GCCISH_CFLAGS_$(1) := -arch i386
  endif
  ifeq (x86_64,$$(HOST_$(1)))
    CFG_GCCISH_CFLAGS_$(1) := -arch x86_64
  endif
  CFG_DSYMUTIL_$(1) := dsymutil
  CFG_DEF_SUFFIX_$(1) := .darwin.def
  # Mac requires this flag to make rpath work
  CFG_INSTALL_NAME_$(1) = -Wl,-install_name,@rpath/$$(1)
  CFG_LIBUV_LINK_FLAGS_$(1) := -lpthread
endif

ifneq ($$(findstring android,$$(OSTYPE_$(1))),)
  CFG_UNIXY_$(1) := 1
  CFG_LIB_NAME_$(1)=lib$$(1).so
  CFG_LIB_GLOB_$(1)=lib$$(1)-*.so
  CFG_GCCISH_CFLAGS_$(1)=-DRUST_NDEBUG -MMD -MP -fPIC -O2 -Wall -g -fno-omit-frame-pointer -D__arm__ -DANDROID -D__ANDROID__
  CFG_GCCISH_LINK_FLAGS_$(1)=-shared -fPIC -ldl -g -lm -lsupc++ -lgnustl_shared
  CFG_GCCISH_DEF_FLAG_$(1)=-Wl,--export-dynamic,--dynamic-list=
  CFG_GCCISH_PRE_LIB_FLAGS_$(1) := -Wl,-whole-archive
  CFG_GCCISH_POST_LIB_FLAGS_$(1) := -Wl,-no-whole-archive -Wl,-znoexecstack
  CFG_DEF_SUFFIX_$(1) := .android.def
  CFG_LIBUV_LINK_FLAGS_$(1) :=
endif

ifdef CFG_UNIXY_$(1)
  CFG_INFO := $$(info cfg: unix-y environment)

  CFG_PATH_MUNGE_$(1) := true
  CFG_EXE_SUFFIX_$(1) :=
  CFG_LDPATH_$(1) :=
  CFG_RUN_$(1)=$$(2)
  CFG_RUN_TARG_$(1)=$$(call CFG_RUN_$(1),,$$(2))

  # FIXME: This is surely super broken
  # ifdef CFG_ENABLE_MINGW_CROSS
  #   CFG_WINDOWSY := 1
  #   CFG_GCCISH_CROSS := i586-mingw32msvc-
  #   ifdef CFG_VALGRIND
  #     CFG_VALGRIND += wine
  #   endif

  #   CFG_GCCISH_CFLAGS := -march=i586
  #   CFG_GCCISH_PRE_LIB_FLAGS :=
  #   CFG_GCCISH_POST_LIB_FLAGS :=
  #   CFG_GCCISH_DEF_FLAG :=
  #   CFG_GCCISH_LINK_FLAGS := -shared

  #   ifeq ($(CFG_CPUTYPE), x86_64)
  #     CFG_GCCISH_CFLAGS += -m32
  #     CFG_GCCISH_LINK_FLAGS += -m32
  #   endif
  # endif
endif

ifneq ($$(findstring mingw,$(OSTYPE_$(1))),)
  CFG_WINDOWSY_$(1) := 1
endif

ifdef CFG_WINDOWSY_$(1)
  CFG_INFO := $$(info cfg: windows-y environment)

  CFG_EXE_SUFFIX_$(1) := .exe
  CFG_LIB_NAME_$(1)=$$(1).dll
  CFG_LIB_GLOB_$(1)=$$(1)-*.dll
  CFG_DEF_SUFFIX_$(1) := .def
ifdef MSYSTEM
  CFG_LDPATH_$(1) :=$$(CFG_LDPATH_$(1)):$$(PATH)
  CFG_RUN_$(1)=PATH="$$(CFG_LDPATH_$(1)):$$(1)" $$(2)
else
  CFG_LDPATH_$(1) :=
  CFG_RUN_$(1)=$$(2)
endif

  CFG_RUN_TARG_$(1)=$$(call CFG_RUN_$(1),$$(HLIB$$(1)_H_$$(CFG_BUILD_TRIPLE)),$$(2))
  CFG_LIBUV_LINK_FLAGS_$(1)=-lWs2_32 -lpsapi -liphlpapi

  ifndef CFG_ENABLE_MINGW_CROSS
    CFG_PATH_MUNGE_$(1) := $$(strip perl -i.bak -p             \
                             -e 's@\\(\S)@/\1@go;'       \
                             -e 's@^/([a-zA-Z])/@\1:/@o;')
    CFG_GCCISH_CFLAGS_$(1) += -march=i686
    CFG_GCCISH_LINK_FLAGS_$(1) += -shared -fPIC
  endif
  CFG_INSTALL_NAME_$(1) =
endif


CFG_INFO := $(info cfg: using $(CFG_C_COMPILER))
ifeq ($(CFG_C_COMPILER),clang)
  ifeq ($(origin CC),default)
    CC_$(1)=clang
  endif
  ifeq ($(origin CXX),default)
    CXX_$(1)=clang++
  endif
  ifeq ($(origin CPP),default)
    CPP_$(1)=clang -E
  endif
  CFG_GCCISH_CFLAGS_$(1) += -Wall -Werror -g
  CFG_GCCISH_CXXFLAGS_$(1) += -fno-rtti
  CFG_GCCISH_LINK_FLAGS_$(1) += -g
  # These flags will cause the compiler to produce a .d file
  # next to the .o file that lists header deps.
  CFG_DEPEND_FLAGS_$(1) = -MMD -MP -MT $$(1) -MF $$(1:%.o=%.d)

  CFG_SPECIFIC_CC_CFLAGS_$(1) = $$(CFG_CLANG_CFLAGS)
  CFG_SPECIFIC_CC_CFLAGS_$(1) += $$(CFG_CLANG_CFLAGS_$(1))

else
ifeq ($(CFG_C_COMPILER),gcc)
  ifeq ($(origin CC),default)
    CC_$(1)=gcc
  endif
  ifeq ($(origin CXX),default)
    CXX_$(1)=g++
  endif
  ifeq ($(origin CPP),default)
    CPP_$(1)=gcc -E
  endif
  CFG_GCCISH_CFLAGS_$(1) += -Wall -Werror -g
  CFG_GCCISH_CXXFLAGS_$(1) += -fno-rtti
  CFG_GCCISH_LINK_FLAGS_$(1) += -g
  # These flags will cause the compiler to produce a .d file
  # next to the .o file that lists header deps.
  CFG_DEPEND_FLAGS_$(1) = -MMD -MP -MT $$(1) -MF $$(1:%.o=%.d)

  CFG_SPECIFIC_CC_CFLAGS_$(1) = $$(CFG_GCC_CFLAGS)
  CFG_SPECIFIC_CC_CFLAGS_$(1) += $$(CFG_GCC_CFLAGS_$(1))

else

  CFG_ERR := $$(error please try on a system with gcc or clang)
endif
endif

AR_$(1)=ar

# Finally, set up the tool chain for the Android cross
ifneq ($$(findstring android,$$(OSTYPE_$(1))),)
  CC_$(1)=$$(CFG_ANDROID_NDK_PATH)/bin/arm-linux-androideabi-gcc
  CXX_$(1)=$$(CFG_ANDROID_NDK_PATH)/bin/arm-linux-androideabi-g++
  CPP_$(1)=$$(CFG_ANDROID_NDK_PATH)/bin/arm-linux-androideabi-gcc -E
  AR_$(1)=$$(CFG_ANDROID_NDK_PATH)/bin/arm-linux-androideabi-ar
  RUSTC_FLAGS_$(1)=--android-ndk-path='$$(CFG_ANDROID_NDK_PATH)'
endif

  CFG_COMPILE_C_$(1) = $$(CC_$(1))  \
        $$(CFG_GCCISH_CFLAGS)             \
        $$(CFG_GCCISH_CFLAGS_$(1))       \
        $$(CFG_SPECIFIC_CC_CFLAGS_$(1))        \
        $$(CFG_DEPEND_FLAGS_$(1))                            \
        -c -o $$(1) $$(2)
    CFG_LINK_C_$(1) = $$(CC_$(1)) \
        $$(CFG_GCCISH_LINK_FLAGS_$(1)) -o $$(1)  \
        $$(CFG_GCCISH_DEF_FLAG_$(1))$$(3) $$(2)      \
        $$(call CFG_INSTALL_NAME_$(1),$$(4))
  CFG_COMPILE_CXX_$(1) = $$(CXX_$(1))  \
        $$(CFG_GCCISH_CFLAGS)             \
        $$(CFG_GCCISH_CFLAGS_$(1))       \
        $$(CFG_GCCISH_CXXFLAGS_$(1))           \
        $$(CFG_SPECIFIC_CC_CFLAGS_$(1))        \
        $$(CFG_DEPEND_FLAGS_$(1))                            \
        -c -o $$(1) $$(2)
    CFG_LINK_CXX_$(1) = $$(CXX_$(1)) \
        $$(CFG_GCCISH_LINK_FLAGS_$(1)) -o $$(1)      \
        $$(CFG_GCCISH_DEF_FLAG_$(1))$$(3) $$(2)      \
        $$(call CFG_INSTALL_NAME_$(1),$$(4))


ifeq ($$(findstring android,$$(OSTYPE_$(1))),)
  # We're using llvm-mc as our assembler because it supports
  # .cfi pseudo-ops on mac
  CFG_ASSEMBLE_$(1)=$$(CPP_$(1)) $$(CFG_DEPEND_FLAGS_$(1)) $$(2) | \
                    $$(LLVM_MC_$$(CFG_BUILD_TRIPLE)) \
                    -assemble \
                    -filetype=obj \
                    -triple=$(1) \
                    -o=$$(1)
else
  # But android uses the Android NDK assembler
  CFG_ASSEMBLE_$(1)=$$(CC_$(1)) $$(CFG_DEPEND_FLAGS_$(1)) $$(2) -c -o $$(1)
endif

endef

$(foreach target,$(CFG_TARGET_TRIPLES),\
  $(eval $(call CFG_MAKE_TOOLCHAIN,$(target))))


