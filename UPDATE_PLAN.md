# Plano de Atualização do Design Source

**Versão:** 1.0
**Data:** 2026-02-14
**Baseado em:** Análise comparativa entre design-source e ollama-oxide (estado atual)

## Contexto

O projeto **design-source** serve como metodologia de referência para desenvolvimento de bibliotecas Rust com suporte de agentes IA. O projeto **ollama-oxide** evoluiu significativamente desde a criação do design-source, introduzindo padrões e decisões arquiteturais que não estão refletidos nas Skills e documentação atuais.

Este plano visa alinhar o design-source com as práticas reais comprovadas no ollama-oxide.

---

## Resumo de Gaps Identificados

| Severidade | Quantidade | Exemplos |
|------------|-----------|----------|
| Alta | 6 | Módulo `primitives` → `inference`, trait-based API, feature flags |
| Média | 7 | Naming `_blocking`, Endpoints constants, conditional compilation |
| Baixa | 3 | Diretórios de specs, README detalhamento |

---

## Etapa 1: Atualização da Skill de Arquitetura

**Arquivo:** `.claude/skills/architecture/SKILL.md`
**Prioridade:** Alta

### Tarefas:

1.1. **Atualizar estrutura de módulos**
- Renomear `src/primitives/` para `src/inference/` em todos os exemplos
- Adicionar módulo `src/tools/` (Feature: "tools")
- Adicionar módulo `src/model/` (Feature: "model")
- Estrutura atualizada:
  ```
  src/
  ├── lib.rs
  ├── error.rs
  ├── inference/    # Feature: "inference" (default)
  ├── http/         # Feature: "http" (default)
  ├── tools/        # Feature: "tools" (opcional)
  ├── model/        # Feature: "model" (opcional)
  └── conveniences/ # Feature: "conveniences" (opcional)
  ```

1.2. **Adicionar seção de Feature Flags**
- Documentar arquitetura de feature flags em 3 níveis:
  - Nível de módulo: `#[cfg(feature = "...")]` em `lib.rs`
  - Nível de campo: `#[cfg(feature = "tools")]` em campos de structs
  - Nível de método: `#[cfg(feature = "model")]` em métodos de traits
- Incluir exemplo de `Cargo.toml` com features
- Documentar cenários de uso (types-only, API client, with tools, full)

1.3. **Atualizar Error Handling Architecture**
- Renomear `LibraryError` para `Error`
- Usar padrão `{Type}Error` nos variants (HttpError, TimeoutError, etc.)
- Remover `#[from]` e usar `impl From<T>` manual
- Adicionar variants: `ConnectionError`, `TimeoutError`, `MaxRetriesExceededError`, `HttpStatusError`

1.4. **Atualizar padrão de configuração do Client**
- Substituir `ClientBuilder` por `ClientConfig` struct com `Default`
- Documentar 3 construtores: `new(config)`, `default()`, `with_base_url(url)`
- Mostrar uso de `Arc<Client>` para thread safety

1.5. **Adicionar seção de Trait-Based API Design**
- Documentar padrão `OllamaApiAsync` / `OllamaApiSync` traits
- Uso de `#[async_trait]` macro
- Separação de traits para async e sync

---

## Etapa 2: Atualização da Skill de Convenções

**Arquivo:** `.claude/skills/conventions/SKILL.md`
**Prioridade:** Alta

### Tarefas:

2.1. **Atualizar naming de Error variants**
- De: `Http(#[from] reqwest::Error)` → Para: `HttpError(String)`
- Documentar padrão `{Type}Error` como convenção do projeto
- Adicionar todos os variants reais: HttpError, HttpStatusError, SerializationError, ApiError, ConnectionError, TimeoutError, MaxRetriesExceededError

2.2. **Adicionar seção de Conditional Compilation**
- Padrões `#[cfg(feature = "...")]` em módulos, structs, métodos
- Exemplos de feature gating em testes e examples
- `required-features` em `[[example]]` no Cargo.toml

2.3. **Atualizar naming de async/sync**
- De: `_sync` suffix → Para: `_blocking` suffix
- Exemplo: `version()` (async) / `version_blocking()` (sync)

2.4. **Adicionar padrão de arquivo único por tipo**
- Documentar convenção: um tipo primário por arquivo
- Naming: `{type_name}.rs` (snake_case do tipo)
- `mod.rs` como facade de re-exports apenas

---

## Etapa 3: Atualização da Skill de API Design

**Arquivo:** `.claude/skills/api-design/SKILL.md`
**Prioridade:** Alta

### Tarefas:

3.1. **Atualizar padrão de Async/Sync**
- Substituir `_sync` por `_blocking` em todos os exemplos
- Documentar trait-based design:
  ```rust
  #[async_trait]
  pub trait OllamaApiAsync: Send + Sync {
      async fn version(&self) -> Result<VersionResponse>;
  }

  pub trait OllamaApiSync: Send + Sync {
      fn version_blocking(&self) -> Result<VersionResponse>;
  }
  ```

3.2. **Adicionar padrão de Endpoints Constants**
- Documentar uso de struct `Endpoints` com constantes `&'static str`
- Substituir exemplos de inline string formatting

3.3. **Adicionar padrão de generic retry helpers**
- `get_with_retry<T>()`, `post_with_retry<R,T>()`, `delete_empty_with_retry<R>()`
- Documentar exponential backoff (100ms × attempt)
- Documentar que 4xx não faz retry

3.4. **Adicionar seção de Type Erasure**
- Padrão `ErasedTool` / `ToolWrapper` para heterogeneous collections
- Bridge entre typed traits e object-safe traits

---

## Etapa 4: Atualização da Skill de Implementação

**Arquivo:** `.claude/skills/implementation/SKILL.md`
**Prioridade:** Média

### Tarefas:

4.1. **Atualizar padrão de implementação de endpoints**
- De: `impl OllamaClient { pub async fn feature() }` direto
- Para: trait-based com `#[async_trait] impl OllamaApiAsync for OllamaClient`

4.2. **Atualizar workflow step 2 (módulo)**
- Renomear `src/primitives/mod.rs` para `src/inference/mod.rs`
- Mostrar escala real (30+ arquivos por módulo)

4.3. **Adicionar padrão de Endpoints constants**
- Mostrar uso de `Endpoints::VERSION` em vez de strings inline

4.4. **Adicionar seção de Feature-Gated Implementation**
- Como implementar métodos condicionais em traits
- Como adicionar campos condicionais em structs
- Exemplos com `#[cfg(feature = "model")]`

---

## Etapa 5: Atualização da Skill de Testing

**Arquivo:** `.claude/skills/testing/SKILL.md`
**Prioridade:** Média

### Tarefas:

5.1. **Adicionar seção de Feature-Gated Tests**
- Como escrever testes para features opcionais
- Exemplo: `#[cfg(feature = "tools")]` em módulos de teste
- Uso de `required-features` em testes de integração

5.2. **Expandir seção de mockito**
- Padrões reais de mock HTTP usados no ollama-oxide
- Exemplos de mock para GET, POST, DELETE
- Padrão de naming: `client_{operation}_tests.rs`

5.3. **Documentar ausência de doc tests**
- Rationale: feature flags tornam doc tests difíceis de manter
- Toda cobertura existe em unit + integration tests

---

## Etapa 6: Atualização da Skill de Documentação

**Arquivo:** `.claude/skills/documentation/SKILL.md`
**Prioridade:** Baixa

### Tarefas:

6.1. **Atualizar referências de diretórios**
- `spec/primitives/` → `spec/apis/`
- `spec/impl-plans/` → `impl/`

6.2. **Adicionar ARCHITECTURE.md ao template**
- Documentar como arquivo obrigatório de documentação

6.3. **Atualizar padrão de naming de examples**
- Padrão: `{feature}_{variant}_{mode}.rs`
- Exemplo: `chat_with_tools_async.rs`, `model_create_async.rs`

---

## Etapa 7: Atualização do CLAUDE.md

**Arquivo:** `CLAUDE.md`
**Prioridade:** Média

### Tarefas:

7.1. **Atualizar seção de File Organization**
- Substituir `src/primitives/` por `src/inference/`
- Adicionar `src/tools/` e `src/model/`
- Remover `requests.rs` / `responses.rs` combinados
- Mostrar padrão de arquivo único por tipo

7.2. **Atualizar referências a módulos**
- Todas as menções a "primitives" devem ser "inference"
- Adicionar referências a tools e model modules

7.3. **Adicionar CONTRIBUTING.md à lista de documentação**

---

## Etapa 8: Atualização dos Templates

**Prioridade:** Baixa

### Tarefas:

8.1. **Atualizar `templates/endpoint-spec.yaml`**
- Adicionar campo `feature` (qual feature flag é necessária)
- Adicionar campo `trait_method` (nome do método no trait)

8.2. **Atualizar `templates/implementation-plan.md`**
- Adicionar seção de Feature Flag Requirements
- Atualizar padrão de implementação para trait-based

---

## Etapa 9: Atualização das Phases

**Prioridade:** Baixa

### Tarefas:

9.1. **Atualizar `phases/phase-1-foundation.md`**
- Incluir setup de feature flags
- Atualizar error types para padrão `{Type}Error`

9.2. **Atualizar `phases/phase-2-primitives.md`**
- Renomear referências de primitives para inference
- Adicionar seção de feature-gated types

9.3. **Atualizar `phases/phase-3-implementation.md`**
- Substituir padrão direto por trait-based API
- Adicionar padrão de generic retry helpers
- Usar `_blocking` em vez de `_sync`

9.4. **Atualizar `phases/phase-4-conveniences.md`**
- Sem alterações significativas identificadas

---

## Etapa 10: Atualização do Modelo Ollama

**Arquivo:** `model/Modelfile`
**Prioridade:** Média

### Tarefas:

10.1. **Atualizar system prompt do Modelfile**
- O system prompt contém a metodologia completa embarcada
- Deve refletir todas as atualizações das etapas 1-9
- Atualizar módulos, naming, patterns, feature flags

---

## Etapa 11: Atualização do Reference Example

**Arquivo:** `examples/ollama-oxide-reference.md`
**Prioridade:** Média

### Tarefas:

11.1. **Atualizar exemplos de código**
- Trocar `primitives` por `inference` em todos os exemplos
- Adicionar exemplos de feature-gated code
- Atualizar error types para padrão real
- Mostrar trait-based API implementation
- Atualizar `_sync` para `_blocking`

---

## Ordem de Execução Sugerida

| Ordem | Etapa | Prioridade | Dependências |
|-------|-------|-----------|--------------|
| 1 | Etapa 1 - Arquitetura | Alta | Nenhuma |
| 2 | Etapa 2 - Convenções | Alta | Nenhuma |
| 3 | Etapa 3 - API Design | Alta | Nenhuma |
| 4 | Etapa 4 - Implementação | Média | Etapa 1 |
| 5 | Etapa 5 - Testing | Média | Etapa 2 |
| 6 | Etapa 7 - CLAUDE.md | Média | Etapas 1-3 |
| 7 | Etapa 11 - Reference | Média | Etapas 1-5 |
| 8 | Etapa 10 - Modelfile | Média | Etapas 1-5 |
| 9 | Etapa 6 - Documentação | Baixa | Nenhuma |
| 10 | Etapa 8 - Templates | Baixa | Etapas 1-4 |
| 11 | Etapa 9 - Phases | Baixa | Etapas 1-5 |

---

## Notas

- Nenhuma implementação será feita sem aprovação prévia do usuário
- Cada etapa pode ser executada independentemente (exceto onde indicado)
- As etapas de prioridade Alta afetam a correção dos padrões documentados
- As etapas de prioridade Média melhoram completude e consistência
- As etapas de prioridade Baixa são refinamentos incrementais
