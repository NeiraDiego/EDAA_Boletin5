#!/bin/bash

# Script para compilar y ejecutar benchmarks de multiplicación de matrices
# con diferentes tamaños N y almacenar resultados en CSV

# Tamaños de matriz a probar
SIZES=(1000 2000 3000 4000 5000 6000)

# Cache line size
CLS=64

# Archivos de salida CSV
MM_CSV="mmNvsT.csv"
MMT_CSV="mmtNvsT.csv"
MMS_CSV="mmsNvsT.csv"

# Compilador y flags
CC=gcc
CFLAGS="-O3 -Wextra -Wall"

echo "Verificando complejidad de las multiplicaciones de matrices"

# Matrix Multiplication (mm)
echo "N,Time(s)" > $MM_CSV
echo "Procesando MatrixMultiplication..."

for N in "${SIZES[@]}"; do
    echo "  Compilando con N=$N..."
    $CC $CFLAGS -DN=$N -o mm.o MatrixMultiplication.c

    if [ $? -eq 0 ]; then
        echo "  Ejecutando con N=$N..."
        TIME=$(./mm.o)
        # Extraer solo el número del tiempo
        TIME_VALUE=$(echo $TIME | awk '{print $1}')
        echo "$N,$TIME_VALUE" >> $MM_CSV
        echo "    Tiempo: $TIME_VALUE segundos"
    else
        echo "    Error en compilación"
    fi
done

# Matrix Multiplication Transposed (mmt)
echo "N,Time(s)" > $MMT_CSV
echo "Procesando MatrixMultiplicationTransposed..."

for N in "${SIZES[@]}"; do
    echo "  Compilando con N=$N..."
    $CC $CFLAGS -DN=$N -o mmt.o MatrixMultiplicationTransposed.c

    if [ $? -eq 0 ]; then
        echo "  Ejecutando con N=$N..."
        TIME=$(./mmt.o)
        TIME_VALUE=$(echo $TIME | awk '{print $1}')
        echo "$N,$TIME_VALUE" >> $MMT_CSV
        echo "    Tiempo: $TIME_VALUE segundos"
    else
        echo "    Error en compilación"
    fi
done

# Matrix Multiplication Submatrix (mms)
echo "N,Time(s)" > $MMS_CSV
echo "Procesando MatrixMultiplicationSubmatrix..."

for N in "${SIZES[@]}"; do
    echo "  Compilando con N=$N..."
    $CC $CFLAGS -DN=$N -DCLS=$CLS -o mms.o MatrixMultiplicationSubmatrix.c

    if [ $? -eq 0 ]; then
        echo "  Ejecutando con N=$N..."
        TIME=$(./mms.o)
        TIME_VALUE=$(echo $TIME | awk '{print $1}')
        echo "$N,$TIME_VALUE" >> $MMS_CSV
        echo "    Tiempo: $TIME_VALUE segundos"
    else
        echo "    Error en compilación"
    fi
done

echo ""
echo "Resultados almacenados en:"
echo "  - $MM_CSV"
echo "  - $MMT_CSV"
echo "  - $MMS_CSV"
