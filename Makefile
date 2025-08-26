CC=g++
CFLAGS=-std=c++11 -O3 -I.
DEPS_HPP = spend/circom.hpp spend/calcwit.hpp proof_of_burn/circom.hpp proof_of_burn/calcwit.hpp fr/fr.hpp
DEPS_O = spend/main.o spend/calcwit.o spend/spend.o proof_of_burn/main.o proof_of_burn/calcwit.o proof_of_burn/proof_of_burn.o fr/fr.o fr/fr_asm.o

ifeq ($(shell uname),Darwin)
	NASM=nasm -fmacho64 --prefix _
endif
ifeq ($(shell uname),Linux)
	NASM=nasm -felf64
endif
	
all: libcircuits.a
	
%.o: %.cpp $(DEPS_HPP)
	$(CC) -c $< $(CFLAGS) -o $@

fr/fr_asm.o: fr/fr.asm
	$(NASM) fr/fr.asm -o fr/fr_asm.o
	
libcircuits.a: $(DEPS_O)
	ar rcs libcircuits.a $(DEPS_O) -lgmp
