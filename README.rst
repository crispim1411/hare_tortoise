Encontrar os duplicados
************************

Dado uma lista contendo n+1 inteiros onde cada inteiro está entre 1 e n (incluso), prove que pelo menos um número duplicado deve existir. 
Assumindo que há apenas um número duplicado, encontre-o.

Obs:
Não modifique o array (read-only)
Use apenas constantes O(1)
Complexidade temporal deverá ser menor que O(n^2).
Há apenas um número repetido mas pode estar repetido mais de uma vez.

Tentativa 1
============
Ordenar a lista e então verificar números adjacentes.
        
Resultado: Muito lento a tarefa de ordenar e depois percorrer a lista.

Tentativa 2
============
Usar hashmap para diminuir complexidade temporal, possui tempo de busca constante.

Resultado: Excedeu o uso da memória, complexidade S(N)

Tentativa 3
============
Usando ciclo de detecção de floyd (tartaruga e a lebre).