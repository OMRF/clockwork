<script>
    import { Button } from '$lib/components/ui/button'
    import { invoke } from '@tauri-apps/api'
    import { open } from '@tauri-apps/api/dialog'

    const click = async () => {
        const response = await invoke('test')

        console.log(response)
    }
</script>

<Button on:click={click}>
    Test
</Button>