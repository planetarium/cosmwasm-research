<template>
  <!-- Not Connected -->
    <!-- Status Display / User Feedback -->
    <div class="title">
      <p>Counter: {{counter}}</p>
    </div>

    <!-- Controls -->
    <div class="button-controls">
      <SpButton type="secondary" @click="getCount()">Get Counter</SpButton>
      <SpButton type="secondary" @click="incrementCounter()">Increment Counter</SpButton>
      <SpButton type="secondary" @click="resetCounter()">Reset Counter</SpButton>
    </div>
    <div>
      <SpAcc></SpAcc>
    </div>

    <!-- Loading -->
    <div class="loading" v-if="loading.status">
      <p v-if="loading.msg">{{loading.msg}}</p>
    </div>

    <div class="logs" v-if="logs.length">
      <div v-for="(log,i) in logs" :key="i">
        <p class="label" v-if="log.timestamp">
          <strong><span v-if="log.executedTxs">Counter Incremented&nbsp</span><span v-if="log.reset">Counter Reset&nbsp</span>({{log.timestamp}}):</strong>
        </p>
        <pre class="log-entry">{{ log }}</pre>
      </div>
    </div>
</template>

<script>
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { calculateFee } from "@cosmjs/stargate"
import { SpAcc } from '@starport/vue'
import { computed, reactive } from 'vue'
import { useStore } from 'vuex'

const ContractAddress = process.env.VUE_APP_CONTRACT_ADDRESS

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
    contract: process.env.VUE_APP_CONTRACT_ADDRESS,
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

.row {
  display: flex;
  flex-wrap: wrap;
}
.col {
  flex-grow: 1;
  padding: 20px;
}
</style>