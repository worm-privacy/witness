.PHONY=clean

all: spend/spend proof_of_burn/proof_of_burn

proof_of_burn/proof_of_burn.dat:
	cd proof_of_burn && unzip proof_of_burn.dat.zip

proof_of_burn/proof_of_burn: proof_of_burn/proof_of_burn.dat
	cd proof_of_burn && make

spend/spend.dat:
	cd spend && unzip spend.dat.zip

spend/spend: spend/spend.dat
	cd spend && make

clean:
	cd proof_of_burn && rm -rf *.o *.dat proof_of_burn
	cd spend && rm -rf *.o *.dat spend