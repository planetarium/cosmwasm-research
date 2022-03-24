<template>
  <div class="container">
    <div class="sp-form-group">
      <div class="sp-form-group">
        <div class="sp-text sp-bold">Admin(optional)</div>
        <div>
          <input
              class="sp-input"
              v-model="admin"
              placeholder="cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7"
          />
        </div>
      </div>
      <div class="sp-form-group">
        <div class="sp-text sp-bold">Code ID</div>
        <div>
          <input
              class="sp-input"
              v-model="codeId"
              placeholder="1"
          />
        </div>
      </div>
      <div class="sp-form-group">
        <div class="sp-text sp-bold">Label</div>
        <div>
          <input
              class="sp-input"
              v-model="label"
              placeholder="A human-readable name for this contract in lists. ex) Marshall Token"
          />
        </div>
      </div>
      <div class="sp-form-group">
        <div class="sp-text sp-bold">Init msg</div>
        <div>
          <textarea
              class="sp-textarea"
              v-model="message"
              placeholder='{
  "name": "Marshall Token",
  "symbol": "MARS",
  "decimals": 6,
  "initial_balances":
  [
    {
      "address": "cosmos1qw9pz5e8shcrz6qkcar6v9tknh0v89jnfq2zv7",
      "amount": "10000000000"
    }
  ]
}'
              style="height: 24rem"
          />
        </div>
      </div>
      <SpButton @click="instantiateCode()">Instantiate a Code</SpButton>
    </div>
    <div class="sp-line"></div>
    <!-- Loading -->
    <div class="sp-text" v-if="loading.status">
      <p v-if="loading.msg">{{loading.msg}}</p>
    </div>

    <div class="sp-text" v-if="contract.length">
      <div v-if="contract"><p class="sp-header-text">Instantiated Contract Address : </p>{{contract}}</div>
    </div>
  </div>
</template>

<script>
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { computed, reactive } from 'vue'
import { useStore } from 'vuex'
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
    loading: {
      status: false,
      msg:""
    },
    logs: [],
    message: "",
    label: "",
    codeId: 0,
    admin: "",
  }),
  methods: {
    instantiateCode: async function () {
      this.loading = {
        status: true,
        msg: "Instantiating contract..."
      }
      let cwClient = await SigningCosmWasmClient.connectWithSigner(this.rpc, this.signer, {gasPrice: this.gasPrice})
      let instantiateResult = await cwClient.instantiate(this.address, Number(this.codeId), JSON.parse(this.message), this.label, "auto" )
      console.log('Instantiate contract', instantiateResult)
      if (instantiateResult.contractAddress.length >0 ) {
        this.contract = instantiateResult.contractAddress
        this.loading.status = false
        this.loading.msg = ""
      } else {
        this.loading.status = true
        this.loading.msg = "Instantiating contract failed due to " + instantiateResult.logs
      }
    },
  }
}
</script>

<style scoped>
.title {
  font-family: Inter, serif;
  font-style: normal;
  font-weight: 600;
  font-size: 28px;
  line-height: 127%;
  /* identical to box height, or 36px */
  letter-spacing: -0.02em;
  font-feature-settings: 'zero';
  color: #000000;
  margin-bottom: 32px;
}

</style>