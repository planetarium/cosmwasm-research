<template>
  <div class="container">
    <div class="sp-form-group">
      <div class="sp-form-group">
        <div class="sp-text sp-bold">Contract Address</div>
        <div>
          <input
              class="sp-input"
              v-model="contract"
              placeholder="cosmos1wn625s4jcmvk0szpl85rj5azkfc6suyvf75q6vrddscjdphtve8swfuffg"
          />
        </div>
      </div>
      <div class="sp-form-group" v-if="contract.length > 0">
        <div class="sp-text sp-bold">Query</div>
        <div>
          <textarea
              class="sp-textarea"
              v-model="message"
              v-bind:placeholder="queryExample"
              style="height: 24rem"
          />
        </div>
      </div>
      <SpButton @click="query()">Query</SpButton>
    </div>
    <div class="sp-line"></div>
    <!-- Loading -->
    <div class="sp-text" v-if="loading.status">
      <p v-if="loading.msg">{{loading.msg}}</p>
    </div>
    <div class="sp-form-group" v-if="queryResult.constructor === Object && Object.keys(queryResult).length > 0">
      <div class="sp-text sp-bold">Query Result</div>
      <div>
          <div class="sp-textarea">
            <p>{{JSON.stringify((queryResult))}}</p>
          </div>
      </div>
    </div>
  </div>
</template>

<script>
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { computed, reactive } from 'vue'
import { useStore } from 'vuex'

const QueryExample = "{\n" +
    "  \"token_info\": {}\n" +
    "}\n" +
    "{\n" +
    "  \"marketing_info\": {}\n" +
    "}\n" +
    "{\n" +
    "  \"download_logo\": {}\n" +
    "}\n" +
    "{\n" +
    "  \"minter\": {}\n" +
    "}\n" +
    "{\n" +
    "  \"balance\": {\n" +
    "    \"address\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\"\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"allowance\": {\n" +
    "    \"owner\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"spender\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\"\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"all_accounts\": {}\n" +
    "}\n" +
    "{\n" +
    "  \"all_allowances\": {\n" +
    "    \"owner\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\"\n" +
    "  }\n" +
    "}"
export default {
  name: "Contract",
  setup() {
    // store
    let $s = useStore()

    // computed
    let address = reactive(computed(() => $s.getters['common/wallet/address']))
    let signer = reactive(computed(() => $s.getters['common/wallet/signer']))
    let wallet = reactive(computed(() => $s.getters['common/wallet/wallet']))
    let gasPrice = computed(() => $s.getters['common/wallet/gasPrice'])
    let rpc = computed(() => $s.getters['common/env/apiTendermint'])

    return {
      address, signer, wallet, gasPrice, rpc
    }
  },
  data: () => ({
    contract: "",
    queryExample: QueryExample,
    loading: {
      status: false,
      msg:""
    },
    message: "",
    queryResult: {},
  }),
  methods: {
    query: async function() {
      let cwClient = await SigningCosmWasmClient.connectWithSigner(this.rpc, this.signer)
      this.loading = {
        status: true,
        msg: "Query data..."
      }
      this.queryResult = await cwClient.queryContractSmart(this.contract, JSON.parse(this.message))
      this.loading.status = false
      this.loading.msg = ""
    },
  }
}
</script>

<style scoped>
</style>