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
        <div class="sp-text sp-bold">Execute Message</div>
        <div>
          <textarea
              class="sp-textarea"
              v-model="message"
              v-bind:placeholder="executeMessageExample"
              style="height: 24rem"
          />
        </div>
      </div>
      <SpButton @click="execute()">Execute</SpButton>
    </div>
    <div class="sp-line"></div>
    <!-- Loading -->
    <div class="sp-text" v-if="loading.status">
      <p v-if="loading.msg">{{loading.msg}}</p>
    </div>

    <div v-if="logs.length">
      <div v-for="(log,i) in logs" :key="i">
        <p v-if="log.timestamp">
          <strong>
            <span v-if="log.executedTxs">Counter Incremented&nbsp</span>
            <span v-if="log.reset">Counter Reset&nbsp</span>
            ({{log.timestamp}}):
          </strong>
        </p>
        <pre>{{ log }}</pre>
      </div>
    </div>
  </div>
</template>

<script>
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { computed, reactive } from 'vue'
import { useStore } from 'vuex'
import {calculateFee} from "@cosmjs/stargate";

const ExecuteMessageExample = "{\n" +
    "  \"transfer\": {\n" +
    "    \"recipient\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"amount\": 1\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"transfer_from\": {\n" +
    "    \"owner\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"recipient\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"amount\": 1\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"mint\": {\n" +
    "    \"recipient\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"amount\": 1\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"burn\": {\n" +
    "    \"amount\": 1\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"burn_from\": {\n" +
    "    \"owner\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"amount\": 1\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"send\": {\n" +
    "    \"contract\": \"cosmos1wn625s4jcmvk0szpl85rj5azkfc6suyvf75q6vrddscjdphtve8swfuffg\",\n" +
    "    \"amount\": 1,\n" +
    "    \"msg\": \"encoded message string\"\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"send_from\": {\n" +
    "    \"owner\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"contract\": \"cosmos1wn625s4jcmvk0szpl85rj5azkfc6suyvf75q6vrddscjdphtve8swfuffg\",\n" +
    "    \"amount\": 1,\n" +
    "    \"msg\": \"encoded message string\"\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"increase_allowance\": {\n" +
    "    \"spender\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"amount\": 1\n" +
    "  }\n" +
    "}\n" +
    "{\n" +
    "  \"decrease_allowance\": {\n" +
    "    \"spender\": \"cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7\",\n" +
    "    \"amount\": 1\n" +
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
    executeMessageExample: ExecuteMessageExample,
    loading: {
      status: false,
      msg:""
    },
    logs: [],
    message: "",
    queryResult: {},
  }),
  methods: {
    execute: async function() {
      let entrypoint = JSON.parse(this.message)
      this.loading = {
        status: true,
        msg: entrypoint
      }
      console.log('Tx args', {
        senderAddress: this.address,
        contractAddress: this.contract,
        msg: entrypoint
      })
      try {
        // Send Tx
        let cwClient = await SigningCosmWasmClient.connectWithSigner(this.rpc, this.signer, {gasPrice: this.gasPrice})
        console.log(this.address, this.contract, entrypoint, "auto")
        let executeResult = await cwClient.execute(this.address, this.contract, entrypoint, "auto")
        this.loading.status = false
        this.loading.msg = ""
        console.log(entrypoint, executeResult)
        // Update Logs
        if (executeResult.logs) {
          if (executeResult.logs.length) {
            this.logs.unshift({
              executedTxs: executeResult,
              timestamp: new Date().getTime()
            })
            console.log('Logs Updated', this.logs)
          }
        }
      } catch (e) {
        console.warn('Error executing message', e)
        this.loading.status = false
        this.loading.msg = ""
      }
    },
  }
}
</script>

<style scoped>
</style>