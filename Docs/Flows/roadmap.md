# Artificial Society - Roadmap de Desenvolvimento

Este documento descreve o plano de desenvolvimento para a "Artificial Society", um sistema de simulação de IA para um MMORPG, construído com o motor Bevy em Rust. O objetivo é criar um mundo dinâmico e emergente, impulsionado por agentes com comportamentos complexos e interações sociais.

## Fase 1: Fundação e Protótipo (✅ Concluído)

O objetivo desta fase era estabelecer a base técnica do projeto, garantir que o ambiente de desenvolvimento estivesse funcional e criar um protótipo mínimo para validar a arquitetura principal.

-   [x] **Configuração do Projeto:** Inicializar o projeto Rust com o Bevy Engine.
-   [x] **Gerenciamento de Dependências:** Configurar e resolver versões compatíveis para `bevy`, `rand` e bibliotecas de depuração.
-   [x] **Arquitetura ECS Básica:** Definir os primeiros Componentes (`Npc`, `Personality`, `KnowledgeBase`) e Sistemas.
-   [x] **Protótipo de Propagação:** Implementar um sistema de simulação mínimo onde uma informação (um "rumor") se espalha através de uma população de NPCs.
-   [x] **Integração de Ferramentas de Depuração:** Integrar com sucesso o `bevy_inspector_egui` para permitir a inspeção de entidades e componentes em tempo real.

## Fase 2: A Mente do Agente - Complexidade Interna

O foco desta fase é enriquecer os agentes individuais, movendo-os de autômatos simples para entidades com estados internos, motivações e conflitos.

-   [ ] **Modelo de Necessidades e Desejos:**
    -   [ ] **Componente `Needs`:** Adicionar necessidades básicas (ex: `fome`, `energia`, `segurança`, `social`).
    -   [ ] **Sistema `NeedsDecay`:** Fazer com que as necessidades decaiam com o tempo.
    -   [ ] **Componente `Desire`:** Gerar um desejo primário (`Desire`) quando uma necessidade atinge um limiar crítico (ex: `fome` alta gera `Desire::FindFood`).
-   [ ] **Máquina de Estados Emocionais (Baseada no Sistema 1):**
    -   [ ] **Componente `EmotionalState`:** Adicionar emoções básicas (ex: `medo`, `raiva`, `alegria`) que são influenciadas por eventos e pelo estado das necessidades.
    -   [ ] **Sistema `EmotionSystem`:** Modular como os eventos (ex: encontrar comida, ser atacado) e as necessidades (ex: fome extrema) afetam o `EmotionalState`.
-   [ ] **Conflito Interno (Instinto vs. Razão - Sistema 1 vs. Sistema 2):**
    -   [ ] **Componente `Willpower`:** Adicionar um recurso de "força de vontade" ou "autocontrole", derivado da `Personality`.
    -   [ ] **Sistema `ImpulseSystem`:** Gerar ações impulsivas baseadas em emoções fortes (ex: `raiva` alta -> `Impulse::Attack`).
    -   [ ] **Sistema `DeliberationSystem`:** O "Sistema 2" que pode gastar `Willpower` para suprimir um `Impulse` e escolher uma ação mais racional. A falha em suprimir leva o agente a agir por instinto.

## Fase 3: A Teia Social - Interações Externas

Com agentes mais complexos, o foco se volta para a qualidade e a profundidade de suas interações sociais.

-   [ ] **Sistema de Reputação e Confiança:**
    -   [ ] **Componente `Reputation`:** Dar a cada NPC um score de reputação.
    -   [ ] **Componente `TrustMatrix`:** Permitir que cada NPC mantenha um registro de quão confiável ele considera os outros NPCs.
    -   [ ] **Sistema `TrustUpdate`:** Modificar a confiança com base em interações. Ações positivas (compartilhar recursos) aumentam a confiança; ações negativas (mentir, roubar) a diminuem.
-   [ ] **Propagação de Informação Avançada:**
    -   [ ] **Entidade `InformationUnit`:** Transformar o "rumor" em uma entidade própria com metadados (`source`, `credibility`, `is_truth`).
    -   [ ] **Refatorar `RumorPropagationSystem`:** Fazer com que a probabilidade de acreditar em uma `InformationUnit` dependa da `Trust` na fonte e da `Personality` do receptor.
    -   [ ] **Sistema de Verificação de Fatos:** Permitir que NPCs possam, ocasionalmente, verificar uma informação através de observação direta, ajustando a `credibility` da informação e a `Trust` na fonte original.
-   [ ] **Formação de Grupos e Relações:**
    -   [ ] **Componente `SocialTies`:** Modelar relações explícitas (`Friend`, `Rival`, `Family`).
    -   [ ] **Sistema `RelationshipSystem`:** Formar e dissolver laços com base em interações repetidas, confiança mútua e traços de personalidade compatíveis.
    -   [ ] **Emergência de Grupos:** Implementar lógica para que NPCs com alta confiança mútua e objetivos semelhantes formem grupos/facções informais.

## Fase 4: Interação com o Mundo e Objetivos de Longo Prazo

Esta fase conecta os agentes ao ambiente de forma significativa e lhes dá a capacidade de formular e perseguir objetivos complexos.

-   [ ] **Interação com o Ambiente:**
    -   [ ] **Componentes de Recursos:** Criar recursos no mundo (ex: `FoodSource`, `Shelter`).
    -   [ ] **Sistema de Ação Ambiental:** Implementar a lógica para que os NPCs interajam com esses recursos para satisfazer suas `Needs`.
-   [ ] **Arquitetura de Objetivos (Behavior Trees ou GOAP):**
    -   [ ] **Pesquisa:** Investigar e escolher uma arquitetura de IA para planejamento (Behavior Trees são uma ótima opção para Bevy).
    -   [ ] **Implementação:** Substituir a lógica de decisão simples por uma árvore de comportamento que permita aos NPCs formular planos de múltiplos passos (ex: para satisfazer a fome, o plano é `IrAtéAFloresta` -> `ProcurarPorFrutas` -> `ColetarFrutas` -> `ComerFrutas`).
-   [ ] **Economia e Propriedade:**
    -   [ ] **Componente `Inventory`:** Dar aos NPCs a capacidade de possuir itens.
    -   [ ] **Sistema de Troca (Barter):** Implementar uma lógica de troca simples entre NPCs, baseada em suas necessidades e na confiança mútua.

## Fase 5: O Caos e o Oculto

A fase final, que adiciona as camadas de intriga e os elementos de fantasia específicos do seu MMORPG.

-   [ ] **Naturezas Ocultas (Vampiro, Lobisomem):**
    -   [ ] **Componente `HiddenNature`:** Adicionar um componente que define a natureza secreta de um NPC.
    -   [ ] **Sistema de Impulsos Sobrenaturais:** Criar impulsos específicos ligados a essas naturezas (ex: `Desire::FeedOnBlood` para vampiros), que entram em conflito com a "máscara humana" através do sistema de `Willpower`.
-   [ ] **Mistérios e Descobertas:**
    -   [ ] **Sistema de Geração de Mistérios:** Criar eventos ou `InformationUnit`s complexas cujas causas não são óbvias.
    -   [ ] **Sistema de Investigação:** Permitir que NPCs com alta `openness` ou `conscientiousness` possam ativamente buscar pistas para resolver mistérios, gerando novas narrativas emergentes.
