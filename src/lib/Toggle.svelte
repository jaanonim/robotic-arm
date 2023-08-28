<script>
    import { Button, Center, Container, Flex, Title } from "@svelteuidev/core";
    import { invoke } from "@tauri-apps/api/tauri";

    export let name = "";
    export let orientation = "row";
    export let error = "";
</script>

<Container
    override={{
        padding: "10px",
        border: "1px #fff solid",
        borderRadius: "10px",
        width: "200px",
    }}
>
    <Center>
        <Title>{name}</Title>
    </Center>
    <Center>
        <Flex direction={orientation}>
            <Button
                override={{ margin: "10px" }}
                on:click={async () => {
                    error = await invoke("up", { name });
                }}
                ><slot name="up" />
            </Button>
            <Button
                override={{ margin: "10px" }}
                on:click={async () => {
                    error = await invoke("zero", { name });
                }}
                ><slot name="down" />
            </Button>
        </Flex>
    </Center>
</Container>
