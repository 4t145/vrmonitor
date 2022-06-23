<template >
  <v-card
    color="lighten-2"
    width=400
  >
    <v-card-title class="text-h5 lighten-3">
      <v-avatar
        color="primary"
        rounded
        size="56"
        tile
      >
        <img
          referrerpolicy="no-referrer"
          :src="$props.liver?.avatar.toString()"
          alt="avatar"
          height="64"
          width="64"
        >
      </v-avatar>
      {{$props.liver?.uname}}
    </v-card-title>
    <v-card-text>

    </v-card-text>
    <v-card-text>

    </v-card-text>
    <v-divider></v-divider>
    <v-card-actions>
      <v-spacer>

      </v-spacer>
    </v-card-actions>
  </v-card>
</template>



<script lang="ts">
import { defineComponent, PropType} from 'vue'
import type { Liver } from '../model/liver'
export default defineComponent({
  props: {
    liver: Object as PropType<Liver>
  },
  name: 'LiverCard',
  data() {
    return {

    }
  }
})
</script>

<style>
.liver-card {
  width: 160px;
}
</style>