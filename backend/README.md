# CApp Backend

Backend da plataforma **CApp**, responsável por fornecer a API utilizada pelo aplicativo móvel e por futuros clientes web.

O backend concentra as regras de negócio da plataforma, incluindo:

- autenticação e identidade acadêmica;
- gerenciamento de Centros Acadêmicos;
- contexto ativo de CA;
- perfis administrativos e permissões;
- notícias;
- eventos;
- demandas;
- materiais acadêmicos;
- eleições;
- FAQ;
- apadrinhamento;
- funcionalidades de governança.

O sistema é desenvolvido em **Rust** seguindo uma arquitetura de **Modular Monolith**, organizada por domínios de negócio e baseada na separação entre regras de negócio, casos de uso e detalhes de infraestrutura.

---

# Arquitetura

O backend é organizado em módulos de negócio:

```
src/

├── modules/
├── infrastructure/
├── api/
├── shared/
└── config/
```

Cada módulo possui suas próprias camadas:

```
module/

├── domain/
├── application/
├── infrastructure/
└── api/
```

O fluxo de dependências segue:

```
API

↓

Application

↓

Domain

↓

Infrastructure
```

---

# Princípios arquiteturais

## Modular Monolith

O backend é implementado como um único serviço dividido internamente em módulos independentes.

Essa abordagem permite:

- reduzir complexidade operacional;
- manter consistência entre funcionalidades;
- compartilhar regras comuns;
- facilitar evolução futura.

Caso o sistema cresça significativamente, módulos específicos podem ser extraídos para serviços independentes.

---

# Separação de responsabilidades

## Domain

Contém as regras de negócio do sistema.

Não possui conhecimento sobre:

- banco de dados;
- HTTP;
- Firebase;
- serviços externos.

Exemplos:

- regras de publicação;
- permissões;
- estados eleitorais;
- validações de negócio.

---

## Application

Contém os casos de uso da aplicação.

Responsável por coordenar operações como:

- criar notícia;
- aprovar publicação;
- cadastrar evento;
- realizar inscrição;
- executar processos eleitorais.

---

## Infrastructure

Contém implementações técnicas.

Exemplos:

- PostgreSQL;
- Firebase Authentication;
- Microsoft Graph;
- armazenamento de arquivos.

---

## API

Responsável pela comunicação externa.

Inclui:

- rotas HTTP;
- handlers;
- DTOs;
- respostas.

---

# Estrutura de diretórios

```
src/

├── modules/
├── infrastructure/
├── api/
├── shared/
└── config/
```

---

# Módulos de negócio

## identity

Responsável pela identidade dos usuários.

Inclui:

- autenticação institucional;
- integração com Firebase Authentication;
- integração com Microsoft;
- provisionamento de usuários;
- dados acadêmicos.

O módulo de identidade representa a identidade institucional do usuário e suas informações acadêmicas.

Ele não representa uma relação de pertencimento a um Centro Acadêmico.

---

## ca

Responsável pelo gerenciamento dos Centros Acadêmicos e das relações administrativas entre usuários e CAs.

Inclui:

- criação e gerenciamento de CAs;
- informações do CA;
- perfis administrativos;
- cargos;
- permissões;
- associações administrativas entre usuários e CAs.

O modelo utilizado é:

```
Usuário

↓

Perfil administrativo

↓

Centro Acadêmico

↓

Permissões
```

Um usuário não pertence necessariamente a um CA. Um usuário pode possuir diferentes perfis administrativos em diferentes CAs.

---

## contexto_ca

Responsável pelo conceito de contexto ativo do aplicativo.

Inclui:

- CA favorito;
- seleção do CA ativo;
- troca de contexto.

O CA favorito é uma preferência de navegação utilizada como contexto inicial da aplicação.

Ele não representa pertencimento a um CA.

Fluxo:

```
Usuário inicia o aplicativo

↓

CA favorito

↓

Contexto ativo

↓

Informações exibidas
```

Durante o uso, o usuário pode alterar o CA ativo para visualizar informações de outros CAs disponíveis.

---

## news

Responsável pelo sistema de notícias.

Inclui:

- visualização de notícias do CA ativo;
- criação de notícias;
- publicação direta;
- proposição;
- homologação;
- compartilhamento entre CAs.

---

## events

Responsável pelo gerenciamento de eventos.

Inclui:

- criação de eventos;
- publicação;
- proposição;
- homologação;
- inscrições;
- controle de vagas.

---

## demands

Responsável pelo sistema de demandas.

Inclui dois contextos principais:

### Demandas institucionais

Relacionadas ao curso e à interação entre estudantes, CA e instituição.

### Demandas do espaço físico do CA

Relacionadas a problemas e necessidades de infraestrutura.

---

## materials

Responsável pelos materiais acadêmicos.

Inclui:

- publicação de materiais;
- arquivos;
- links;
- organização por disciplinas;
- acesso pelos estudantes.

---

## faq

Responsável pela base de conhecimento.

Inclui:

- cadastro de perguntas frequentes;
- consulta;
- suporte futuro a chatbot.

---

## elections

Responsável pelo sistema eleitoral.

Inclui:

- comissão eleitoral;
- configuração de eleições;
- chapas;
- candidatos;
- votação;
- apuração;
- criação de nova gestão.

---

## governance

Responsável pelas funcionalidades de transparência e participação.

Inclui:

- histórico de gestões;
- prestação de contas;
- consultas estudantis;
- mecanismos de governança.

---

## mentorship

Responsável pelo programa de apadrinhamento.

Inclui:

- participantes;
- padrinhos;
- afilhados;
- pareamentos;
- critérios de matching.

---

## strikes

Responsável pelo gerenciamento de strikes.

Inclui:

- registro de strikes individuais;
- registro de strikes associados a CAs;
- configuração de regras e consequências.

---

# Infraestrutura compartilhada

Localizada em:

```
src/infrastructure/
```

Contém recursos utilizados por múltiplos módulos.

---

## database

Responsável pelo acesso ao PostgreSQL.

Inclui:

- pool de conexões;
- transações;
- persistência.

---

## firebase

Responsável pela integração com Firebase Authentication.

Responsabilidades:

- validação de tokens;
- identificação de usuários autenticados.

Não contém regras de autorização da aplicação.

---

## microsoft

Responsável pela integração com Microsoft Graph.

Utilizado para obtenção de informações acadêmicas.

---

## storage

Responsável pelo armazenamento de arquivos.

Exemplos:

- PDFs;
- documentos;
- imagens.

---

## notifications

Responsável por mecanismos de comunicação.

Exemplos:

- notificações push;
- integrações futuras.

---

# API global

Localizada em:

```
src/api/
```

Responsável por:

- registro de rotas;
- middleware;
- respostas padronizadas.

---

# Shared

Localizada em:

```
src/shared/
```

Contém código compartilhado entre módulos.

Exemplos:

- erros comuns;
- tipos reutilizados;
- paginação;
- utilidades genéricas.

---

# Configuração

Localizada em:

```
src/config/
```

Responsável por carregar configurações externas.

Exemplos:

```
DATABASE_URL
FIREBASE_PROJECT_ID
MICROSOFT_CLIENT_ID
```

---

# Banco de dados

As alterações do banco são controladas por migrations.

Exemplo:

```
migrations/

001_initial_schema.sql
002_create_news.sql
003_create_events.sql
```

Cada migration representa uma evolução versionada do banco.

---

# Desenvolvimento

Executar aplicação:

```bash
cargo run
```

Executar testes:

```bash
cargo test
```

---

# BDD

O projeto utiliza Behavior Driven Development.

Os arquivos de especificação comportamental ficam no diretório raiz do monorepo:

```
features/
```

Cada funcionalidade possui:

- descrição do comportamento esperado;
- cenários Gherkin;
- implementações de teste específicas para cada cliente.

O backend implementa os testes comportamentais relacionados às regras de negócio e API.

---

# Tecnologias

| Tecnologia | Uso |
|---|---|
| Rust | Backend |
| Axum | Framework HTTP |
| Tokio | Runtime assíncrono |
| SQLx | Banco de dados |
| PostgreSQL | Persistência |
| Firebase Authentication | Identidade |
| Microsoft Graph | Dados acadêmicos |
| Docker | Ambiente |

---

# Filosofia

O backend é uma API independente dos clientes.

O aplicativo Flutter é apenas um consumidor da API.

Futuramente, um frontend web poderá utilizar a mesma infraestrutura sem duplicar regras de negócio.

```
              Rust Backend

                    ↑

        ┌───────────┴───────────┐

        │                       │

Flutter Mobile             Web Frontend
```

As regras de negócio permanecem centralizadas no backend, enquanto diferentes clientes implementam apenas suas respectivas interfaces.
