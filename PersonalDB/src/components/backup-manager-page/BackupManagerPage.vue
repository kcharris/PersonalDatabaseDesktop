<script setup lang="ts">
    import { VSheet} from "vuetify/components";
    import { ref, onMounted} from "vue";
    import { invoke } from "@tauri-apps/api/tauri";
    import BackupResorePopup from "./BackupRestorePopup.vue"
    import BackupRenamePopup from "./BackupRenamePopup.vue"
    import BackupDeletePopup from "./BackupDeletePopup.vue"
    import BackupSuccessPopup from "./BackupSuccessPopup.vue"

    onMounted(()=>{
        refresh_backup_filenames();
    })
    const backup_names = ref<string[]>([])
    const success_dialog = ref(false)

    async function refresh_backup_filenames(){
        let filenames: string[] = await invoke("get_backup_filenames")
        backup_names.value = filenames
    }

    async function renameDB(payload: any){
        await invoke("rename_backup", payload)
        refresh_backup_filenames();
    }

    async function deleteDB(name: string){
        await invoke("delete_backup_file", {backupName: name})
        refresh_backup_filenames();
    }

    async function restoreDB(payload: any){
        let res = await invoke("restore_db", payload)
        if (res === true){
            success_dialog.value = true
        }
    }

</script>


<template>    
    <BackupSuccessPopup v-model="success_dialog"></BackupSuccessPopup>
    <v-sheet color="teal-lighten-4" class="fill-height mx-auto w-100">
        <v-container>
            <v-toolbar color="blue-grey-lighten-5 w-66 mx-auto" density="compact">
                <p class="ml-5">Backup Management</p>
                <v-spacer/>
            </v-toolbar>
            <v-card v-if="backup_names.length == 0" class="w-66 mx-auto" height="85dvh">
                <v-card-title>No Backups found</v-card-title>
            </v-card>
            <v-data-table hide-default-footer height="85dvh" v-else class="w-66 overflow-x-auto mx-auto">
                <thead>
                    <tr>
                        <th >
                            Backup Name
                        </th>
                        <th >
                            Restore
                        </th>
                        <th >
                            Rename
                        </th>
                        <th >
                            Delete
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr
                        v-for="name in backup_names"
                        :key="name"
                    >
                        <td>
                            {{ name }}
                        </td>
                        <td>
                            <BackupResorePopup :backup_name="name" @restore="restoreDB"></BackupResorePopup>
                        </td>
                        <td>
                            <BackupRenamePopup :backup_name="name" @rename="renameDB"></BackupRenamePopup>
                        </td>
                        <td>
                            <BackupDeletePopup :backup_name="name" @delete="deleteDB"></BackupDeletePopup>
                        </td>
                    </tr>
                </tbody>
            </v-data-table>
        </v-container>
    </v-sheet>
</template>

<style>
    th{
        text-align: left;
    }
</style>