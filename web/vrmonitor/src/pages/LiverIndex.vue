
<template>
    <LiverCard v-for="liver in livers" v-bind:key="liver.roomid" :liver="liver"/>
</template>

<script lang="ts">
import { defineComponent} from 'vue'

import LiverCard from '../components/LiverCard.vue'
import { FEED_LIST } from '../model/liver'
export default defineComponent({
  name: 'LiverIndex',
  components: {
    LiverCard
  },
  data() {
    return {
        livers: FEED_LIST
    }
  }
})
</script>