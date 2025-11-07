# Snake Game com Rust
Este projeto foi criado com o objetivo de desenvolver minha lógica de programação e aprofundar meus conhecimentos na linguagem de programação Rust e o uso de bibliotecas externas do rust

## Conceitos aplicados
- 🏠 **Ownership e Barrowing**: O Rust funciona alocando espaços na memória que são apenas deles próprios e para que seja possível usar esses espaços alocados para funções ou outras váriaveis para receberem o mesmo valor, é preciso emprestar. Um exemplo interessante é como se seu código Rust fosse um condomínio fechado com cada váriavel sendo dona de seu próprio apartamento, outras variáveis não podem entrar sem pedir a permissão da variável dona do apartamento, e quando isso ocorre, por "bons modos" a variável dona não poderá modificar seu apartamento até que a variável estrangeira deixe seu apartamento.
- 🖩 **Patterns Matting**: Em outras linguagens, sua aproximação mais semelhante é o ```Switch/case```, em rust é chamado de ```match```, sendo esse uma versão muito mais poderosa, podendo conter valores diferentes para tipos de dados semelhantes e declarar funções inteiras dentro de seu escopo.
- ❌ **Result e Option**: No Rust, os conceitos de entrada de váriaveis e conversão devem ser feitos utilizando os ```.unwrap()```, ```.expect()``` ou até mesmo o ```match``` para capturar possíveis erros, sendo este uma versão do catch para vriaveis.
- 📏 **Vetores**: Vetores são de ultima análise, um tipo de array em Rust, sendo estes sua variante dinâmica sem um tamanho pré-definido, são declaradas como ```vec![]```, sua tipagem pode ser ou não definida, caso não seja definida na hora da declaração da váriavel, será inferida baseada no primeiro elemento.
- 🔁 **loop**: São uma forma while porém sem a necessidade de uma comparação, podendo ser infinito, seu conceito pode ser copiado em outras linguagens como ```while(true)```.
- 🗺️ **enum**: São um determiado tipo agrupamento de outros tipos diferentes para uma variável, sendo um conceito mais amplo que o uso de ```struct```, podendo conter tipos de dados diferentes um dos outros sem a necessidade de declarar váruios ```struct```, uma forma semelhante em outras linguagens são as interfaces que mapeiam determinados tipos de uma váriavel.

## Bibliotecas usadas
- 🎲 **Rand**: Biblioteca para geração de valores aleatórios.
- 📓 **io**: É uma biblioteca padrão do Rust para entrada de variáveis.
- ⬆️⬇️⬅️➡️ **Crossterm**: Biblioteca para captura de macros das teclas e manipulação do terminal.
- 💤 **thread**: Biblioteca padrão do Rust, permite configuração de multithreading, pausa no código, entre outros.
- ⌛ **time**: Outra biblioteca padrão do Rust, usada para definir um determinado periodo, normalmente usado junto a sleep para definir um periodo de pausa.

## Como foi feito
1. **Criação do Grid**: O primeiro objetivo foi fazer um tabuleiro de 31x31 utilizando um array fixo e enums para determinar cada tipo de célula da grid e então exibila como uma grid pontilhada:
![Grade do jogo](./assets/images/grid.png)

2. **Criando a cobrinha**: para criar a cobrinha, foi feito usando um vetor de tamanho variavem que era renderizado dentro da array fixa da grid por um enum de ```Cell::Snake```