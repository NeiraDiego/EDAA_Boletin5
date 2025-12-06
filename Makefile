MY_C=gcc
C_FLAGS=-O3 -Wextra -Wall

# Cache line size. Puede ser obtenida por medio del comando "getconf
# LEVEL1_DCACHE_LINESIZE" (en Linux)
CLS=64

# Size of the square matriz
N=1024

mm: MatrixMultiplication.c
	@echo "[BLD] Matrix multiplication"
	@$(MY_C) $(C_FLAGS) -DN=$(N) -o mm.o MatrixMultiplication.c

mmt: MatrixMultiplicationTransposed.c
	@echo "[BLD] Matrix multiplication transposed"
	@$(MY_C) $(C_FLAGS) -DN=$(N) -o mmt.o MatrixMultiplicationTransposed.c


mms: MatrixMultiplicationSubmatrix.c
	@echo "[BLD] Matrix multiplication submatrix"
	@$(MY_C) $(C_FLAGS) -DN=$(N) -DCLS=$(CLS) -o mms.o MatrixMultiplicationSubmatrix.c
