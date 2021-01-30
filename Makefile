CFLAGS = -std=c18 -Wall -Wextra -Wpedantic -Werror=vla -O3 -g -flto

all: hanachan

hanachan: bsp.o dump.o main.o mat34.o player.o quat.o rkg.o util.o vec3.o vec4.o wii.o yaz.o
	$(CC) $(CFLAGS) $^ -o $@ -lm

bsp.o: bsp.c bsp.h vec3.h util.h
dump.o: dump.c dump.h vec3.h util.h vec4.h
main.o: main.c dump.h vec3.h util.h vec4.h player.h bsp.h rkg.h
mat34.o: mat34.c mat34.h vec3.h util.h vec4.h wii.h
player.o: player.c mat34.h vec3.h util.h vec4.h quat.h wii.h
quat.o: quat.c quat.h vec3.h util.h vec4.h
rkg.o: rkg.c rkg.h util.h yaz.h
util.o: util.c util.h
vec3.o: vec3.c vec3.h util.h wii.h
vec4.o: vec4.c vec4.h util.h wii.h
wii.o: wii.c wii.h util.h wii_tables.h
yaz.o: yaz.c yaz.h util.h

.PHONY: clean
clean:
	$(RM) hanachan *.o
