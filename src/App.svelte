<script lang="ts">
    import {
        Center,
        SimpleGrid,
        Space,
        SvelteUIProvider,
    } from "@svelteuidev/core";
    import Controls from "./lib/Controls.svelte";
    import "iconify-icon";
    import Toggle from "./lib/Toggle.svelte";

    const data = [
        {
            name: "grip",
            upIcon: "material-symbols:compare-arrows-rounded",
            downIcon: "material-symbols:arrows-outward-rounded",
            orientation: "column",
        },
        {
            name: "wrist",
            upIcon: "material-symbols:arrow-upward-alt-rounded",
            downIcon: "material-symbols:fitbit-arrow-downward-rounded",
            orientation: "column",
        },
        {
            name: "elbow",
            upIcon: "material-symbols:arrow-upward-alt-rounded",
            downIcon: "material-symbols:fitbit-arrow-downward-rounded",
            orientation: "column",
        },
        {
            name: "shoulder",
            upIcon: "material-symbols:arrow-upward-alt-rounded",
            downIcon: "material-symbols:fitbit-arrow-downward-rounded",
            orientation: "column",
            flipped: true,
        },
        {
            name: "rotation",
            upIcon: "material-symbols:arrow-left-alt-rounded",
            downIcon: "material-symbols:arrow-right-alt-rounded",
            orientation: "row",
            flipped: true,
        },
    ];
    let error = "";
</script>

<SvelteUIProvider themeObserver="dark">
    <main class="container">
        <SimpleGrid cols={3}>
            {#each data as ele}
                <Controls
                    name={ele.name}
                    orientation={ele.orientation}
                    bind:error
                    flipped={ele.flipped}
                >
                    <iconify-icon
                        style="font-size: 2rem;"
                        slot="up"
                        icon={ele.upIcon}
                    />
                    <iconify-icon
                        style="font-size: 2rem;"
                        slot="down"
                        icon={ele.downIcon}
                    />
                </Controls>
            {/each}
            <Toggle name="led" bind:error
                ><iconify-icon
                    style="font-size: 2rem;"
                    slot="up"
                    icon="material-symbols:flashlight-on-rounded"
                />
                <iconify-icon
                    style="font-size: 2rem;"
                    slot="down"
                    icon="material-symbols:flashlight-off-rounded"
                /></Toggle
            >
        </SimpleGrid><br />
        <br />
        <Center><span>{error}</span></Center>
    </main>
</SvelteUIProvider>

<style>
    span {
        color: red;
    }
</style>
