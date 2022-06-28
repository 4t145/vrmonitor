<script lang="ts">
import { h } from "vue";
import { Component } from 'vue';
import { defineComponent } from 'vue'

import NotFound from './pages/NotFound.vue'
import DanmakuHistory from './pages/Stat/DanmakuHistory.vue'
import LiverIndex from './pages/LiverIndex.vue'

const ROUTE = {
  '': {
    '': LiverIndex,
    'livers': LiverIndex,
    'stat': {
      'danmaku': DanmakuHistory
    }
  }
}
export default defineComponent({
  name: 'App',

  computed: {
    page(): Component {
      let routes = this.current_route.split('/');
      console.log(routes);
      let target:any|Component = ROUTE;
      for(const next of routes) {
        target = target[next];
        if (!target) {
          return NotFound
        }
      }
      return target
    }
  },
  data () {
    return {
      current_route: window.location.pathname,
    }
  },

  render() {
    return h(this.page)
  }
})
</script>