# ICMC CC

Compilador de C para o [processador do ICMC](https://github.com/simoesusp/Processador-ICMC/).

## Manual

### Regras
O compilador segue todas as regras comuns do C99, exceto nos seguintes casos:
- Variáveis globais não podem ser inicializadas.
- Apenas os tipos `void`, `char` e `int` são suportados, sendo que os tipos `char` e `int` são iguais (inteiros de 16 bits).

O compilador também suporta os operadores especiais `inchar` e `outchar`:
- `inchar`: retorna o código da tecla pressionada no instante em que o operador foi chamado, retornando 255 caso nenhuma tecla tenha sido pressionada.
- `outchar <char>, <pos>`: imprime o caractere `char` na posição da tela especificada por `pos`.

### Diretivas de compilação
O compilador suporta as seguintes diretivas:
- `#include`: inclui outros arquivos para a compilação.
- `#define`: define macros.
- `__LINE__`: macro especial que expande para o número da linha no arquivo (expandindo para um número inteiro).
- `/* */`: comenta um bloco.
- `//`: comenta uma linha.

### Bibliotecas padrão
O compilador oferece uma coleção de bibliotecas padrão otimizadas:
- `io.h`: funções de entrada e saída.
- `string.h`: funções de manipulação de arrays.

Confira os [cabeçalhos das bibliotecas](std/) para uma documentação aprofundada.

### Considerações
- Em casos onde performance é necessária, prefira o uso de variáveis globais em vez de variáveis locais e argumentos.
- No momento, há um limite de 6 argumentos por função.

## Convenções
O compilador opera com as seguintes convenções:
- O registrador `r0` é reservado e armazena o endereço da base da stack da função atual.
- O registrador `r7` é reservado para operações com valores imediatos, operações com a base da stack e retorno de funções.
- Funções recebem argumentos através de registradores, indo do `r1` até `r6`.
- Funções usam `r7` para o valor de retorno.
