#!/bin/bash
./ASTARStowage.sh ./ASTAR-tests mapa1 contenedores1 heuristica1
./ASTARStowage.sh ./ASTAR-tests mapa1 contenedores2 heuristica1
./ASTARStowage.sh ./ASTAR-tests mapa1 contenedores1 heuristica2
./ASTARStowage.sh ./ASTAR-tests mapa1 contenedores2 heuristica2
./ASTARStowage.sh ./ASTAR-tests mapa1 contenedores1 sin-heuristica
./ASTARStowage.sh ./ASTAR-tests mapa1 contenedores2 sin-heuristica
./ASTARStowage.sh ./ASTAR-tests mapa2 contenedores3 heuristica1
./ASTARStowage.sh ./ASTAR-tests mapa2 contenedores4 heuristica1
./ASTARStowage.sh ./ASTAR-tests mapa2 contenedores3 heuristica2
./ASTARStowage.sh ./ASTAR-tests mapa2 contenedores4 heuristica2
./ASTARStowage.sh ./ASTAR-tests mapa2 contenedores3 sin-heuristica
./ASTARStowage.sh ./ASTAR-tests mapa2 contenedores4 sin-heuristica
