#include "witness/include/blobstore.h"
#include "witness/src/main.rs.h"

/// We need this accessor since cxx doesn't support hashmaps yet
class IOSignalInfoAccessor {
private:
  Circom_CalcWit *calcWitContext;

public:
  explicit IOSignalInfoAccessor(Circom_CalcWit *calcWit)
      : calcWitContext(calcWit) {}
  auto operator[](size_t index) const -> decltype(auto) {
    return (calcWitContext
                ->templateInsId2IOSignalInfoList)[index % get_size_of_io_map()];
  }
};

typedef void (*Circom_TemplateFunction)(uint __cIdx, Circom_CalcWit *__ctx);

//////////////////////////////////////////////////////////////////
/// Generated code from circom compiler below
//////////////////////////////////////////////////////////////////

void Multiplier2_0_create(uint soffset, uint coffset, Circom_CalcWit *ctx,
                          rust::string componentName, uint componentFather);
void Multiplier2_0_run(uint ctx_index, Circom_CalcWit *ctx);
Circom_TemplateFunction _functionTable[1] = {Multiplier2_0_run};
Circom_TemplateFunction _functionTableParallel[1] = {NULL};
uint get_main_input_signal_start() { return 2; }

uint get_main_input_signal_no() { return 2; }

uint get_total_signal_no() { return 4; }

uint get_number_of_components() { return 1; }

uint get_size_of_input_hashmap() { return 256; }

uint get_size_of_witness() { return 4; }

uint get_size_of_constants() { return 1; }

uint get_size_of_io_map() { return 0; }

void release_memory_component(Circom_CalcWit *ctx, uint pos) {
  {}
}

// function declarations
// template declarations
void Multiplier2_0_create(uint soffset, uint coffset, Circom_CalcWit *ctx,
                          rust::string componentName, uint componentFather) {
  ctx->componentMemory[coffset].templateId = 0;
  ctx->componentMemory[coffset].templateName = "Multiplier2";
  ctx->componentMemory[coffset].signalStart = soffset;
  ctx->componentMemory[coffset].inputCounter = 2;
  ctx->componentMemory[coffset].componentName = componentName;
  ctx->componentMemory[coffset].idFather = componentFather;
  ctx->componentMemory[coffset].subcomponents = rust::Vec<uint32_t>{};
}

void Multiplier2_0_run(uint ctx_index, Circom_CalcWit *ctx) {
  rust::Vec<FrElement> &signalValues = ctx->signalValues;
  u64 mySignalStart = ctx->componentMemory[ctx_index].signalStart;
  rust::string myTemplateName = ctx->componentMemory[ctx_index].templateName;
  rust::string myComponentName = ctx->componentMemory[ctx_index].componentName;
  u64 myFather = ctx->componentMemory[ctx_index].idFather;
  u64 myId = ctx_index;
  rust::Vec<u32> mySubcomponents =
      ctx->componentMemory[ctx_index].subcomponents;
  rust::Vec<FrElement> &circuitConstants = ctx->circuitConstants;
  rust::Vec<rust::string> &listOfTemplateMessages = ctx->listOfTemplateMessages;
  rust::Vec<FrElement> expaux = create_vec(4);
  rust::Vec<FrElement> lvar = create_vec(0);
  uint sub_component_aux;
  {
    FrElement *aux_dest = &signalValues[mySignalStart + 0];
    // load src
    Fr_mul(&expaux[1], &signalValues[mySignalStart + 1],
           &signalValues[mySignalStart + 2]);             // line circom 7
    Fr_add(&expaux[0], &expaux[1], &circuitConstants[0]); // line circom 7
    // end load src
    Fr_copy(aux_dest, &expaux[0]);
  }
  for (uint i = 0; i < 0; i++) {
    uint index_subc = ctx->componentMemory[ctx_index].subcomponents[i];
    release_memory_component(ctx, index_subc);
  }
}

void run(Circom_CalcWit *ctx) {
  Multiplier2_0_create(1, 0, ctx, "main", 0);
  Multiplier2_0_run(0, ctx);
}