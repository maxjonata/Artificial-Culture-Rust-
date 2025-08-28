# Arquitetura de Integração - Artificial Society

## Visão Geral da Arquitetura

O sistema "Artificial Society" implementa uma simulação multi-agente complexa usando Bevy ECS, onde cada NPC é uma entidade composta por múltiplos componentes que representam diferentes aspectos de sua existência. A arquitetura é baseada em três princípios fundamentais:

1. **Separação de Responsabilidades**: Cada componente representa um domínio específico (neurológico, cognitivo, social, etc.)
2. **Emergência através de Interação**: Comportamentos complexos emergem da interação entre sistemas simples
3. **Feedback Loops Caóticos**: Hypergraphs criam retroalimentação não-linear entre componentes

## Estrutura Hierárquica dos Sistemas

```
┌─────────────────────────────────────────────────────────────┐
│                    CAMADA EMERGENTE                         │
│  - Detecção de Comportamentos Emergentes                   │
│  - Transições de Fase                                      │
│  - Métricas Coletivas                                      │
└─────────────────────────────────────────────────────────────┘
                                ↕
┌─────────────────────────────────────────────────────────────┐
│                   CAMADA HIPERGRAPH                        │
│  - Nós de Hipergraph                                       │
│  - Arestas Dinâmicas                                       │
│  - Propagação de Ativação                                  │
│  - Padrões Temporais Caóticos                              │
└─────────────────────────────────────────────────────────────┘
                                ↕
┌─────────────────────────────────────────────────────────────┐
│                   CAMADA SOCIAL-CULTURAL                   │
│  - Redes de Confiança                                      │
│  - Propagação de Informação                                │
│  - Dinâmicas de Scapegoating                               │
│  - Normas e Cultura                                        │
└─────────────────────────────────────────────────────────────┘
                                ↕
┌─────────────────────────────────────────────────────────────┐
│                   CAMADA COGNITIVA                         │
│  - Sistema de Crenças                                      │
│  - Processamento Dual (S1/S2)                              │
│  - Tomada de Decisão                                       │
│  - Memória e Aprendizado                                   │
└─────────────────────────────────────────────────────────────┘
                                ↕
┌─────────────────────────────────────────────────────────────┐
│                   CAMADA NEUROLÓGICA                       │
│  - Neurotransmissores                                      │
│  - Carga Alostática                                        │
│  - Sistema de Força de Vontade                             │
│  - Núcleo Emocional                                        │
└─────────────────────────────────────────────────────────────┘
```

## Fluxos de Dados Principais

### 1. Fluxo Ascendente (Bottom-Up)
```
Neurotransmissores → Emoções → Cognição → Comportamento Social → Emergência
```

### 2. Fluxo Descendente (Top-Down)
```
Pressão Social → Normas Culturais → Dissonância Cognitiva → Estresse → Neuroquímica
```

### 3. Fluxo Lateral (Hypergraph)
```
Qualquer Componente ↔ Hypergraph ↔ Qual