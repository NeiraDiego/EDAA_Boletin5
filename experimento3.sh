#!/bin/bash

# Experimento 3: Análisis de métricas IPC y L1 cache vs tamaño de matriz
# Compila y ejecuta los tres programas con diferentes tamaños N
# Mide: Instructions, IPC, L1-dcache loads, L1-dcache misses, L1 miss rate

SIZES=(256 512 1024 2048)
REPETITIONS=30
EVENTS="cycles,instructions,L1-dcache-loads,L1-dcache-load-misses"
CC=gcc
CFLAGS="-O3 -Wextra -Wall"
CLS=64

echo "========================================="
echo "Experimento 3: IPC y L1 Cache Analysis"
echo "Tamaños: ${SIZES[*]}"
echo "Repeticiones: $REPETITIONS"
echo "========================================="
echo ""

# Función para extraer valor de evento del CSV de perf
extract_value() {
    local csv_file=$1
    local event=$2
    # Buscar la línea que contiene el evento y extraer el primer campo (valor)
    grep "$event" "$csv_file" | head -1 | cut -d, -f1
}

# Para cada programa
for PROG in mm mmt mms; do
    CSV_OUTPUT="${PROG}-exp3.csv"

    echo "========================================="
    echo "Procesando: $PROG"
    echo "========================================="

    # Crear header del CSV de salida
    echo "N,Instructions,Cycles,IPC,L1_dcache_loads,L1_dcache_misses,L1_miss_rate" > $CSV_OUTPUT

    for N in "${SIZES[@]}"; do
        echo ""
        echo "  Compilando $PROG con N=$N..."

        # Compilar según el programa
        if [ "$PROG" = "mms" ]; then
            $CC $CFLAGS -DN=$N -DCLS=$CLS -o ${PROG}.o MatrixMultiplicationSubmatrix.c
        elif [ "$PROG" = "mmt" ]; then
            $CC $CFLAGS -DN=$N -o ${PROG}.o MatrixMultiplicationTransposed.c
        else
            $CC $CFLAGS -DN=$N -o ${PROG}.o MatrixMultiplication.c
        fi

        if [ $? -ne 0 ]; then
            echo "    ERROR: Falló la compilación"
            continue
        fi

        echo "  Ejecutando benchmarks ($REPETITIONS repeticiones)..."

        # Ejecutar perf stat con los eventos específicos
        perf stat -r $REPETITIONS -x , -e $EVENTS ./${PROG}.o 2> temp_perf.csv

        if [ $? -ne 0 ]; then
            echo "    ERROR: Falló la ejecución de perf"
            rm -f temp_perf.csv
            continue
        fi

        # Extraer valores de los eventos
        instructions=$(extract_value temp_perf.csv "instructions")
        cycles=$(extract_value temp_perf.csv "cycles")
        l1_loads=$(extract_value temp_perf.csv "L1-dcache-loads")
        l1_misses=$(extract_value temp_perf.csv "L1-dcache-load-misses")

        # Verificar que se extrajeron valores válidos
        if [ -z "$instructions" ] || [ -z "$cycles" ] || [ -z "$l1_loads" ] || [ -z "$l1_misses" ]; then
            echo "    ERROR: No se pudieron extraer todos los valores"
            echo "    Instructions: $instructions"
            echo "    Cycles: $cycles"
            echo "    L1-loads: $l1_loads"
            echo "    L1-misses: $l1_misses"
            rm -f temp_perf.csv
            continue
        fi

        # Calcular métricas derivadas
        ipc=$(echo "scale=6; $instructions / $cycles" | bc)
        miss_rate=$(echo "scale=6; $l1_misses / $l1_loads * 100" | bc)

        # Escribir al CSV de salida
        echo "$N,$instructions,$cycles,$ipc,$l1_loads,$l1_misses,$miss_rate" >> $CSV_OUTPUT

        echo "    Instructions: $instructions"
        echo "    Cycles: $cycles"
        echo "    IPC: $ipc"
        echo "    L1 loads: $l1_loads"
        echo "    L1 misses: $l1_misses"
        echo "    L1 miss rate: $miss_rate%"

        # Limpiar archivo temporal
        rm -f temp_perf.csv
    done

    echo ""
    echo "Resultados guardados en: $CSV_OUTPUT"
    echo ""
done

echo "========================================="
echo "Experimento completado!"
echo ""
echo "Archivos generados:"
echo "  - mm-exp3.csv"
echo "  - mmt-exp3.csv"
echo "  - mms-exp3.csv"
echo ""
echo "Para generar gráficos LaTeX:"
echo "  rustc parser_exp3.rs -o parser_exp3"
echo "  ./parser_exp3 mm-exp3.csv mmt-exp3.csv mms-exp3.csv > graficos_exp3.tex"
echo "========================================="
