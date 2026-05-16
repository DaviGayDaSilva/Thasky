```markdown
# 🛠️ Thasky

> Local AI. Optimized for reality.

Thasky é uma framework CLI minimalista e ultrarrápida escrita em Rust, projetada especificamente para extrair a máxima performance de modelos de Inteligência Artificial locais (LLMs) em hardware severamente limitado, computadores antigos e dispositivos móveis (como Android via Termux).

A framework **não possui modelos próprios**. Ela atua como um runtime e orquestrador inteligente de alta eficiência para arquivos nos formatos **GGUF**, **ONNX** e **safetensors** variando de 0.5B a 7B parâmetros.

---

## ⚡ Recursos Core

*   **Detecção Automática de Hardware:** Identifica topologias de CPU (incluindo arquiteturas ARM BIG.little), barramentos de memória e APIs gráficas disponíveis (Vulkan/OpenCL).
*   **Gerenciamento Térmico Dinâmico:** Monitora a temperatura do dispositivo em tempo real e introduz micro-escalonamentos para evitar thermal throttling e travamentos.
*   **Lazy Loading via MMAP:** Mapeamento de memória cirúrgico que carrega camadas do modelo sob demanda, permitindo rodar arquiteturas maiores do que a memória RAM física disponível.
*   **Evicção de Cache KV Inteligente:** Monitora a oscilação da memória RAM e reduz dinamicamente a janela de contexto para impedir o acionamento do OOM (Out of Memory) Killer.
*   **Sistema de Patches On-the-Fly:** Permite acoplar conjuntos de dados extras (JSON, CSV, TXT, Markdown) via busca vetorial local e RAG sem a necessidade de re-treinar ou realizar fine-tuning nos pesos do modelo.

---

## 💻 Instalação

### Pré-requisitos
*   Rust (MSRV 1.75+)
*   C/C++ Compiler (para vinculação dos backends GGML/llama.cpp)

```bash
git clone [https://github.com/thasky-ai/thasky.git](https://github.com/thasky-ai/thasky.git)
cd thasky
cargo build --release

```
O binário otimizado estará disponível em ./target/release/thasky.
## 🚀 Interface de Linha de Comando (CLI)
```
thasky [COMMAND] [ARGS]

```
### Comandos Disponíveis
| Comando | Descrição |
|---|---|
| thasky import <PATH> | Registra e analisa estruturalmente um modelo local (.gguf, .onnx). |
| thasky optimize <MODEL> --level <LEVEL> | Executa otimização estática e quantização direcionada ao hardware atual. |
| thasky run <MODEL> [--patch <PATCH_NAME>] | Inicia o chat interativo em tempo real com telemetria cyberpunk nativa. |
| thasky doctor | Avalia a saúde do hardware, gargalos do sistema e limites térmicos. |
| thasky benchmark <MODEL> | Executa um teste de estresse de velocidade de inferência (tokens por segundo). |
| thasky patch <ACTION> | Gerencia (cria, ativa ou desativa) pacotes de contexto e dados locais. |
| thasky models | Lista os modelos importados, tamanhos de arquivo e status de otimização. |
## 📖 Exemplos de Uso
### 1. Importar e Preparar um Modelo
```bash
$ thasky import models/phi3-3b.gguf

```
### 2. Otimização Extrema para Dispositivos Fracos
```bash
$ thasky optimize phi3 --level extreme

```
### 3. Criar e Vincular um Patch de Dados (RAG Local)
```bash
$ thasky patch create coding-pack ./docs/my_api_spec.json
$ thasky patch enable coding-pack

```
### 4. Executar a Inferência Otimizada
```bash
$ thasky run phi3

```
## 🎨 Estética do Terminal
A interface foi projetada sob a filosofia *performance-first* com elementos visuais minimalistas inspirados no estilo cyberpunk industrial:
```
┌── [THASKY RUNTIME] ──────────────────────────────────────┐
🔥 Modelo: phi3 (Otimizado) | 🟢 Patch: coding-pack [Ativo]
🧠 Threads: 4 Ativas | Cache: Ativado | Modo Térmico: Auto
────────────────────────────────────────────────────────────
User> Como otimizar loops em Rust?
Thasky> ⚡ [Inferindo - 21.4 tok/s] [RAM: 74%] [TEMP: 48°C]

```
## 📄 Licença
Distribuído sob a licença MIT. Veja LICENSE para mais informações.
```

```
