# Scheduling Job

Dado um array de "jobs" para execução, no qual cada posição possui um objeto com os seguintes atributos:

1) **ID:** Identificação do job;
2) **Descrição:** Descrição do job;
3) **Data máxima de conclusão do job:** Data máxima em que o job deve ser concluído;
4) **Tempo estimado:** Tempo estimado de execução do job.

Criar um algoritmo que retorne um conjunto de arrays com as seguintes características:

1) Cada array do conjunto representa uma lista de jobs a serem executados em sequência;
2) Cada array deve conter jobs que sejam executados em, no máximo, 8h;
3) Deve ser respeitada a data máxima de conclusão do job;
4) Todos os jobs devem ser executados dentro da janela de execução (data de início e fim).

_**Exemplo de massa de dados:**_

Janela de execução: 2019-11-10 09:00:00 até 2019-11-11 12:00:00.

```json
[
  {
    "ID": 1,
    "Descrição": "Importação de arquivos de fundos",
    "Data máxima de conclusão": "2019-11-10 12:00:00",
    "Tempo estimado": "2 horas"
  },
  {
    "ID": 2,
    "Descrição": "Importação de dados da Base Legada",
    "Data máxima de conclusão": "2019-11-11 12:00:00",
    "Tempo estimado": "4 horas"
  },
  {
    "ID": 3,
    "Descrição": "Importação de dados de integração",
    "Data máxima de conclusão": "2019-11-11 08:00:00",
    "Tempo estimado": "6 horas"
  }
]
```

Output esperado:

```json
[
  [1, 3],
  [2]
]
```
