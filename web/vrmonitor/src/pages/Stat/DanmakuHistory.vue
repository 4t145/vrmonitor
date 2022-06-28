<template>
    <v-card>
        <v-card-title>
            历史弹幕查询
        </v-card-title>
        <v-card-content>
            <v-row
                justify="space-between"
                md="4"
            >  
                <v-text-field
                    type = "number"
                    v-model="filter.roomid"
                    :rules="[rules.u64]"
                    label="房间id"
                ></v-text-field>
            </v-row>  
            <v-row
                justify="space-between"
                md="4"
            >  
                <v-text-field
                    label="从"
                    v-model="filter.time_from"
                    :rules="[rules.timestamp]"

                    type = "datetime-local"
                    placeholder="Placeholder"
                ></v-text-field>
                <v-text-field
                    label="到"
                    v-model="filter.time_to"
                    :rules="[rules.timestamp]"

                    type = "datetime-local"
                    placeholder="Placeholder"
                ></v-text-field>
            </v-row>  
            <v-row
                justify="space-between"
                md="4"
            >  
                <v-checkbox-btn
                    v-model="filter.uid.enable"

                    placeholder="Placeholder"
                    class="pr-2"
                ></v-checkbox-btn>
                指定uid
                <v-text-field 
                    :disabled="!filter.uid.enable"
                    hide-details
                    v-model="filter.uid.uid"
                    label="uid"
                ></v-text-field>
    
            </v-row>  
        </v-card-content>
        <v-card-actions>
            <v-btn prepend prepend-icon="mdi-magnify" @click="fetch_data">
                搜索
            </v-btn>
        </v-card-actions>
    </v-card>
    <v-container>
        <v-table>
            <thead>
                <tr>
                    <th class="text-left">
                    记录时间
                    </th>
                    <th class="text-left">
                    用户名
                    </th>
                    <th class="text-left">
                    UID
                    </th>
                    <th class="text-left">
                    消息
                    </th>

                </tr>
            </thead>
            <tbody>
                <tr v-for="(danmaku, index) in danmakus" v-bind:key = "index">
                    <td>{{danmaku.time}}</td>
                    <td>{{danmaku.data.user.uname}}</td>
                    <td>{{danmaku.data.user.uid}}</td>
                    <td>{{danmaku.data.message.tag==='Plain'?danmaku.data.message.data.message:danmaku.data.message.data.alt_message}}</td>
                </tr>
            </tbody>
        </v-table>
    </v-container>

</template>
<style>

</style>
<script lang="ts">
    import { defineComponent, PropType} from 'vue'
    import { fetch_danmaku_history } from '../../api/query'
    import type { ExtendedEvent } from '../../api/query'
    import { DanmakuEvent, Event } from "bilive-danmaku-json";
    import moment from "moment";
    export default defineComponent({
        data: () => ({
            rules: {
                u64(n:number) {
                    return (Number.isInteger(n) && n > 0) || "请输入一个正整数"
                },
                timestamp(s:string) {
                    return moment(s).unix() > 0 || "请输入正确的时间"
                }
            },
            filter: {
                roomid: undefined as number|undefined,
                time_from: "",
                time_to: "",
                uid: {
                    enable: false,
                    uid: 0
                }
            },
            loading: false,
            date: (new Date(Date.now() - (new Date()).getTimezoneOffset() * 60000)).toISOString().substr(0, 10),
            danmakus: [] as ExtendedEvent<DanmakuEvent> []
        }),
        methods: {
            fetch_data() {
                if(this.filter.roomid!==undefined) {
                    const roomid = this.filter.roomid;
                    const from = moment(this.filter.time_from).unix();
                    const to = moment(this.filter.time_to).unix();
                    console.log(from);
                    console.log(to);
                    let query = {
                        'user': undefined as number|undefined,
                        'time_from': from,
                        'time_to': to,
                        'limit': 100,
                        'skip': 0
                    };
                    if(this.filter.uid) {

                        query["user"] = this.filter.uid.uid
                    }
                    this.loading = true;

                    fetch_danmaku_history(roomid, query).then(
                        resp => {
                            this.danmakus = resp;
                            this.loading = false;
                        }
                    );
                }
            }
        }
    })
</script>