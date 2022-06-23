<template>
  <v-app>
    <v-app-bar app>
      维阿信
    </v-app-bar>
    <v-main>
      <v-navigation-drawer>

      <v-list
        dense
        nav
      >
        <v-list-item
          v-for="item in items"
          :key="item.title"
          link
        >
          <!-- <v-list-item-icon>
            <v-icon>{{ item.icon }}</v-icon>
          </v-list-item-icon> -->

          <v-list-item-content>
            <v-list-item-title>{{ item.title }}</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
      </v-navigation-drawer>

      <v-content>
        <LiverCard v-for="liver in livers" v-bind:key="liver.uid" :liver="liver"/>
      </v-content>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import LiverCard from './components/LiverCard.vue'

export default defineComponent({
  name: 'App',

  components: {
    LiverCard
},

  data () {
    return {
      items: [
        { title: 'Dashboard', icon: 'mdi-view-dashboard' },
        { title: 'Photos',    icon: 'mdi-image' },
        { title: 'About',     icon: 'mdi-help-box' },
      ],
      livers: [
        { uid: 10000, uname: "索维尔sower", roomid: 0, avatar: new URL("https://i2.hdslb.com/bfs/face/9a0b05e498c2a2eabb336267179c0e3b865ece6d.jpg@150w_150h.jpg"), living: false},
        { uid: 10000, uname: "Demo", roomid: 0, avatar: new URL("https://i2.hdslb.com/bfs/face/9a0b05e498c2a2eabb336267179c0e3b865ece6d.jpg@150w_150h.jpg"), living: false},
        { uid: 10000, uname: "Demo", roomid: 0, avatar: new URL("https://i2.hdslb.com/bfs/face/9a0b05e498c2a2eabb336267179c0e3b865ece6d.jpg@150w_150h.jpg"), living: false},
        { uid: 10000, uname: "Demo", roomid: 0, avatar: new URL("https://i2.hdslb.com/bfs/face/9a0b05e498c2a2eabb336267179c0e3b865ece6d.jpg@150w_150h.jpg"), living: false},
      ]
    }
  },
})
</script>
