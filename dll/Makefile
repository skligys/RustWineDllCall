all:

PROJ = libbug64

CFLAGS = -g -O2 \
  -fPIC \
  -Wall \
  -pipe \
  -fno-strict-aliasing \
  -Wempty-body \
  -Wignored-qualifiers \
  -Wstrict-prototypes \
  -Wtype-limits \
  -Wunused-but-set-parameter \
  -Wvla \
  -Wwrite-strings \
  -Wpointer-arith \
  -Wlogical-op \
  -Wno-format-truncation \
  -gdwarf-2 \
  -gstrict-dwarf \
  -fno-omit-frame-pointer

LDFLAGS =
WINE_INCLUDE = /usr/include/wine/windows

CC = gcc
WINEGCC = winegcc
WINEBUILD = winebuild

$(OBJS):%.o: ../../src/%.c
	$(CC) -c -o $@ $^ -I. -I$(WINE_INCLUDE) -D__WINESRC__ -D_REENTRANT $(CFLAGS)

libbug64.dll.so libbug64.dll.fake: libbug64.spec bug64.o
	$(WINEGCC) -o $@ -B$(WINEBUILD) -fasynchronous-unwind-tables -shared libbug64.spec \
  bug64.o $(LDFLAGS)

libbug64.def: libbug64.spec
	$(WINEBUILD) -w --def -o $@ --export libbug64.spec

all: libbug64.dll.so libbug64.dll.fake libbug64.def

clean::
	rm -f *.def *.a *.o *.so *.fake

.PHONY: all clean
