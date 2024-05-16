# ICMC CC

Compilador de C para o [processador do ICMC](https://github.com/simoesusp/Processador-ICMC/).

## Manual

### Regras
O compilador segue todas as regras comuns do C99, exceto nos seguintes casos:
- Variáveis globais não podem ser inicializadas.
- Apenas os tipos `void`, `char` e `int` são suportados, sendo que os tipos `char` e `int` são iguais (inteiros de 16 bits).

O compilador também suporta 2 operadores especiais: `inchar` e `outchar`:
- `inchar`: retorna o código da tecla pressionada no instante em que o operador foi chamado, retornando 255 caso nenhuma tecla tenha sido pressionada.
- `outchar <char>, <pos>`: imprime o caractere `char` na posição da tela especificada por `pos`.

### Diretivas de compilação
O compilador suporta as seguintes diretivas:
- `#include`: incluí outros arquivos para a compilação.
- `#define`: define macros.
- `__LINE__`: macro que expande para o número da linha no arquivo (expandindo para um número inteiro).

### Considerações
- Prefira o uso de variáveis globais em vez de variáveis locais e argumentos (melhor performance).
- No momento há um limite de 6 argumentos por função.
