<template>
  <!-- Not Connected -->
    <!-- Status Display / User Feedback -->
  <div class="container">
    <div class="title">
      <p>Counter: {{counter}}</p>
    </div>

  <div class="sp-form-group">
    <div class="sp-text sp-bold">Counter Contract Address</div>
    <div>
      <input
          class="sp-input"
          v-model="contract"
          placeholder="cosmos1436kxs0w2es6xlqpp9rd35e3d0cjnw4sv8j3a7483sgks29jqwgsks67u5"
      />
    </div>
  </div>


  <!-- Controls -->
    <div class="sp-form-group" v-if="this.contract.length > 0">
      <SpButton @click="getCount()">Get Counter</SpButton>
      <div class="sp-line"></div>
      <SpButton type="secondary" @click="incrementCounter()">Increment Counter</SpButton>
      <SpButton type="secondary" @click="resetCounter()">Reset Counter</SpButton>
    </div>

    <!-- Loading -->
    <div v-if="loading.status">
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
import { calculateFee } from "@cosmjs/stargate"
import { SpAcc } from '@starport/vue'
import { computed, reactive } from 'vue'
import { useStore } from 'vuex'

export default {
  name: "Counter",
  components: {SpAcc},
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
    counter: 0,
    contract: "",
    loading: {status: false, msg:""},
    logs: [],
  }),
  methods: {
    query: async function(entrypoint) {
      let cwClient = await SigningCosmWasmClient.connectWithSigner(this.rpc, this.signer)
      this.loading = {
        status: true,
        msg: "Refreshing counter..."
      }
      return await cwClient.queryContractSmart(this.contract, entrypoint)
    },
    getCount: async function () {
      let entrypoint = {
        get_count: {}
      }
      let query_result = await this.query(entrypoint)
      console.log('Counter Queried', query_result)
      this.loading.status = false
      this.loading.msg = ""
      this.counter = query_result.count
    },

    executeTx: async function(entrypoint) {
      if (!this.wallet) {
        console.warn('Error getting user', this.wallet)
        return
      }
      this.loading = {
        status: true,
        msg: entrypoint
      }
      let txFee = calculateFee(300000, this.gasPrice)
      console.log('Tx args', {
        senderAddress: this.address,
        contractAddress: this.contract,
        msg: entrypoint,
        fee: txFee
      })
      try {
        // Send Tx
        let cwClient = await SigningCosmWasmClient.connectWithSigner(this.rpc, this.signer)
        console.log(this.address, this.contract, entrypoint, txFee)
        let executeResult = await cwClient.execute(this.address, this.contract, entrypoint, txFee)
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
        // Refresh counter display
        await this.getCount()
      } catch (e) {
        console.warn('Error executing Increment', e)
        this.loading.status = false
        this.loading.msg = ""
      }
    },
    incrementCounter: async function () {
      let entrypoint = {
        increment: {}
      }
      await this.executeTx(entrypoint)
    },
    resetCounter: async function () {
      let entrypoint = {
        reset: {
          count: 0
        }
      }
      await this.executeTx(entrypoint)
    }
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

pre {
  line-height:1.2em;
  background:linear-gradient(180deg,#ccc 0,#ccc 1.2em,#eee 0);
  background-size:2.4em 2.4em;
  background-origin:content-box;
  padding: 1em;
  text-align:justify;
  display: inline-block;
  color: #0a4862;
  background-color: #73c8eb;
  border-color: #3bb3e3;
  border-radius: 0.5em;
}
</style>