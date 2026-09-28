<script lang="ts">
	import { Accordion, AccordionItem, Button, Checkbox, Form, FormGroup, Select, SelectItem, TextInput, Disclosure } from "carbon-components-svelte"
	import Play from "carbon-icons-svelte/lib/Play.svelte"
	import Pause from "carbon-icons-svelte/lib/Pause.svelte"
	import Stop from "carbon-icons-svelte/lib/Stop.svelte"
	import { help } from "$lib/helpray"
	import { trackEvent } from "$lib/utils"

	type LauncherState = "stopped" | "starting" | "running" | "paused" | "stopping"
	type SdkStatus = "disconnected" | "connecting" | "connected" | "error"

	export async function handleRequest(request: any) {
		switch (request?.type) {
			default:
				break
		}
	}

	let selectedGameLabel = "Hitman 3"
	let launcherState: LauncherState = "stopped"
	let sdkStatus: SdkStatus = "disconnected"
	let startBenchmark = false
	let benchmarkSceneIndex = "1"
	let resolution = "1920x1080"
	let fullscreen = true
	let monitor = "0"
	let limitMultiGpu = false
	let disableTempAllocReuse = false
	let disableAsyncCompute = false
	let leakVirtualBackbuffer = false
	let debugBackbuffer = false
	let disableDiscard = false
	let fullMemoryDump = false
	let enableDeviceTrim = false
	let deviceTrimWaitCopyQueues = "0"
	let vrAllowInstantSwitch = false

	function setRunningState(nextState: LauncherState, nextStatus: SdkStatus, message: string) {
		launcherState = nextState
		sdkStatus = nextStatus
	}

	function openGame() {
		trackEvent("Open selected game")
		setRunningState("running", "connecting", "Starting the selected game and waiting for the SDK to come online.")
	}

	function pauseGame() {
		trackEvent("Pause selected game")
		setRunningState("paused", "connected", "The game is paused.")
	}

	function closeGame() {
		trackEvent("Close selected game")
		setRunningState("stopped", "disconnected", "The game is closed.")
	}

	$: sdkStatusLabel = sdkStatus === "connected" ? "Connected" : sdkStatus === "connecting" ? "Connecting" : sdkStatus === "error" ? "Error" : "Disconnected"
	$: sdkStatusTone = sdkStatus === "connected" ? "bg-emerald-400" : sdkStatus === "connecting" ? "bg-amber-400" : sdkStatus === "error" ? "bg-red-400" : "bg-neutral-500"

	function setBenchmarkMode(enabled: boolean) {
		startBenchmark = enabled
		benchmarkSceneIndex = enabled ? benchmarkSceneIndex || "1" : benchmarkSceneIndex
	}

	function handleBenchmarkModeChange(event: Event) {
		setBenchmarkMode((event.currentTarget as HTMLInputElement).checked)
	}
</script>

<div class="w-full h-full overflow-y-auto p-3" use:help={{ title: "Game launcher", description: "" }}>
	<div class="mx-auto flex w-full flex-col gap-2 px-1 py-2">
		<section class="pt-1">
			<div class="flex items-center justify-between gap-4">
				<div class="min-w-0">
					<p class="text-[10px] uppercase tracking-[0.3em] text-neutral-500">Game launcher</p>
					<div class="mt-1 break-words text-sm text-neutral-100">{selectedGameLabel}</div>
				</div>
				<div class="shrink-0 text-right text-[10px] uppercase tracking-[0.22em] text-neutral-500">
					<div>SDK status</div>
					<div class="mt-1 flex items-center justify-end gap-2 text-neutral-400 normal-case tracking-normal">
						<span class={`h-2 w-2 rounded-full ${sdkStatusTone}`}></span>
						{sdkStatusLabel}
					</div>
				</div>
			</div>
		</section>

		<div class="flex items-center gap-1">
			<Button size="small" kind="ghost" icon={Play} iconDescription="Play" on:click={openGame} disabled={launcherState === "running" || launcherState === "starting"} />
			<Button size="small" kind="ghost" icon={Pause} iconDescription="Pause" on:click={pauseGame} disabled={launcherState !== "running"} />
			<Button size="small" kind="ghost" icon={Stop} iconDescription="Stop" on:click={closeGame} disabled={launcherState === "stopped" || launcherState === "stopping"} />
		</div>

		<Disclosure summary="Launch options">
			<Accordion flush>
				<AccordionItem title="Benchmark">
					<Checkbox labelText="Enable benchmark" checked={startBenchmark} on:change={handleBenchmarkModeChange} />
					<Select size="sm" inline labelText="Benchmark scene" bind:value={benchmarkSceneIndex} disabled={!startBenchmark}>
						<SelectItem value="1" text="Scene 1" />
						<SelectItem value="2" text="Scene 2" />
						<SelectItem value="3" text="Scene 3" />
					</Select>
					<Checkbox labelText="Quit after benchmark" checked={startBenchmark} disabled />
				</AccordionItem>
				<AccordionItem title="Display">
					<Checkbox labelText="Fullscreen" bind:checked={fullscreen} />
					<TextInput size="sm" inline labelText="Resolution" disabled={fullscreen} bind:value={resolution} />
					<TextInput size="sm" inline labelText="Monitor" bind:value={monitor} />
				</AccordionItem>
				<AccordionItem title="Rendering">
					<Checkbox labelText="Limit multi GPU" bind:checked={limitMultiGpu} />
					<Checkbox labelText="Disable temp allocation reuse" bind:checked={disableTempAllocReuse} />
					<Checkbox labelText="Disable async compute" bind:checked={disableAsyncCompute} />
					<Checkbox labelText="Leak virtual backbuffer" bind:checked={leakVirtualBackbuffer} />
					<Checkbox labelText="Debug backbuffer" bind:checked={debugBackbuffer} />
					<Checkbox labelText="Disable discard" bind:checked={disableDiscard} />
				</AccordionItem>
				<AccordionItem title="Memory and trim">
					<Checkbox labelText="Full memory dump" bind:checked={fullMemoryDump} />
					<Checkbox labelText="Enable device trim" bind:checked={enableDeviceTrim} />
					<TextInput size="sm" inline labelText="Device trim wait copy queues" bind:value={deviceTrimWaitCopyQueues} />
				</AccordionItem>
				<AccordionItem title="Virtual reality">
					<Checkbox labelText="Allow instant switch" bind:checked={vrAllowInstantSwitch} />
				</AccordionItem>
			</Accordion>
		</Disclosure>
	</div>
</div>
