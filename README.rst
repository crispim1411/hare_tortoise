Encontrar os duplicados
************************

Dado uma lista contendo n+1 inteiros onde cada inteiro está entre 1 e n (incluso), prove que pelo menos um número duplicado deve existir. 
Assumindo que há apenas um número duplicado, encontre-o.

Obs:

Não modifique o array (read-only)

Use apenas constantes, memória O(1)

Complexidade temporal deverá ser menor que O(n^2).

Há apenas um número repetido mas pode estar repetido mais de uma vez.

Prova
------
De acordo com o Princípio da casa dos pombos serão n+1 itens para para n números (de 1 a n). Pelo menos um número irá ser escrito duas vezes.

Tentativa 1
============
Ordenar a lista e então verificar números adjacentes.
        
Resultado: Muito lento a tarefa de ordenar e depois percorrer a lista.

Tentativa 2
============
Usar hashmap para diminuir complexidade temporal, possui tempo de busca constante.

Resultado: Apesar da busca constante excedeu o uso da memória ao montar o hashmap.

Tentativa 3
============
Usando ciclo de detecção de floyd (tartaruga e a lebre).

Resultado: Busca linear e uso da memória constante. 
