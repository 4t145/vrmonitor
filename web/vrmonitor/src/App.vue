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
        <Router/>
      </v-content>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { h } from "vue";
import { Component } from 'vue';
import { defineComponent } from 'vue'

import NotFound from './pages/NotFound.vue'
import LiverIndex from './pages/LiverIndex.vue'
import Router from './Router.vue'


export default defineComponent({
  name: 'App',
  components: {
    Router
  },
  computed: {
    
  },
  data () {
    return {
      current_route: window.location.pathname,
      items: [
        { title: 'Dashboard', icon: 'mdi-view-dashboard' },
        { title: 'Photos',    icon: 'mdi-image' },
        { title: 'About',     icon: 'mdi-help-box' },
      ],
    }
  },
})
</script>
