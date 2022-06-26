<template >
  <v-card
    color="lighten-2"
    width=400
  >
    <v-card-title class="text-h5 lighten-3">
      <v-card-avatar>
        <img
          referrerpolicy="no-referrer"
          :src="user_info?.face"
          alt="avatar"
          height="64"
          width="64"
        >
      </v-card-avatar>
      {{user_info?.name}}
      <!-- {{$props.liver?.uname}} -->
    </v-card-title>
    <v-card-content>
      <v-card-text >
        <span v-if="live_status">
          开播中，已播{{live_time}}
        </span>
        <span v-else>
          未开播
        </span>
      </v-card-text>
      <v-chip-group>
        <v-chip v-for="tag in $props.liver?.tags" :key="tag">
          {{tag}}
        </v-chip>
      </v-chip-group>
    </v-card-content>
      <v-divider></v-divider>

    <v-card-actions>
        <v-btn
          class="ml-2 mt-3"
          fab
          icon
          height="40px"
          right
          width="40px"
        >
          <v-icon>mdi-poll</v-icon>
        </v-btn>
    </v-card-actions>
  </v-card>
</template>



<script lang="ts">
import { defineComponent, PropType} from 'vue'
import type { Liver } from '../model/liver'
import { fetch_room_info, fetch_user_info  } from '../api/proxy'
import type {RoomInfo, UserInfo} from '../api/proxy'
import {time_hm} from '../utils'
export default defineComponent({
  props: {
    liver: Object as PropType<Liver>
  },
  name: 'LiverCard',
  data() {
    return {
      room_info: null as (RoomInfo|null),
      user_info: null as (UserInfo|null)
    }
  },
  computed: {
    live_status():boolean {
      return this.room_info?.live_status===1
    },
    live_time():string {
      if(this.room_info) {
        let time_now = Math.floor(new Date().getTime()/1000);
        let time = time_hm(time_now - this.room_info.live_time)
        if(time.hour === 0) {
          return `${time.minute}分钟`
        } else {
          return `${time.hour}小时${time.minute}分钟`
        }
      } else {
        return ``
      }

    }
  },
  methods: {
    refresh_room_info() {
      if (this.$props.liver) {
        fetch_room_info(this.$props.liver.roomid).then(room_info => {this.room_info = room_info});
      }
    },
    refresh_user_info() {
      if (this.$props.liver) {
        fetch_user_info(this.$props.liver.uid).then(user_info => {this.user_info = user_info});
      }
    }
  },
  created() {
    this.refresh_room_info();
    this.refresh_user_info();
  },
})
</script>

<style>
.liver-card {
  width: 160px;
}
</style>