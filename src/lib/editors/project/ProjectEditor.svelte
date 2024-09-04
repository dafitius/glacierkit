<script lang="ts">
	import type { ProjectRequest } from "$lib/bindings-types"
	import { help } from "$lib/helpray"
	import {
		TextInput,
		Button,
		TextArea,
		Checkbox,
		Select,
		SelectItem,
		Tabs,
		Tab,
		TabContent,
		Tile,
		FluidForm,
		FileUploaderItem,
		Form,
		ClickableTile,
		OverflowMenu,
		OverflowMenuItem,
		Accordion,
		AccordionItem,
		ExpandableTile
	} from "carbon-components-svelte"
	import { CheckboxCheckedFilled, Close, CloseOutline, TrashCan } from "carbon-icons-svelte"

	export let id: string
	let project_path: String

	export async function handleRequest(request: ProjectRequest) {
		console.log(`Project editor ${id} handling request`, request)

		switch (request.type) {
			case "initialise":
				project_path = request.data.project_path
				break
			default:
				break
		}
	}

	let config = {
		/* -------------------------------------- Required data -------------------------------------- */
		id: "Atampy26.ExampleMod", // Unique ID; recommended to use reverse URI style (Author.Mod)
		name: "Example Mod",
		description: "It is a mod",
		authors: ["Atampy26", "No one else"],
		version: "1.0.0", // The mod's version, used to compare against the linked JSON - make sure to use semantic versioning (Major.Minor.Patch)
		frameworkVersion: "2.33.19", // The framework version the mod is designed for

		/* -------------------------------------- Optional data -------------------------------------- */
		updateCheck: "https://hitman-resources.netlify.app/framework/updates/exampleMod.json", // A JSON (see Mod Updates) that will be checked for updates (MUST BE HTTPS)

		/* ------- This data can be used in mod options as well as on the top level (optional) ------- */
		contentFolders: ["content"], // Folders next to the manifest to use for the mod content
		blobsFolders: ["blobs"], // Folders next to the manifest to use for blobs (new/edited existing JSON and GFXI files)
		localisation: {
			english: {
				UI_THEBESTMOD: "The Best Mod" // You can use UI_THEBESTMOD elsewhere
			},
			french: {
				UI_THEBESTMOD: "Le Meilleur Mod idk french lmao" // UI_THEBESTMOD will automatically translate depending on language
			},
			italian: {},
			german: {},
			spanish: {},
			russian: {},
			chineseSimplified: {},
			chineseTraditional: {},
			japanese: {}
		},
		localisationOverrides: {
			// Allows you to override specific localisation in specific files
			// NOTE: Localisation string hashes should be entered as decimal numbers, hex numbers from HMLanguageTools will NOT work
			"00123456789ABCDE": {
				english: {
					"123456789": "The framework is overriding this text"
				},
				french: {
					"123456789": "Le framework est en train de overriding this text"
				},
				italian: {},
				german: {},
				spanish: {},
				russian: {},
				chineseSimplified: {},
				chineseTraditional: {},
				japanese: {}
			}
		},
		localisedLines: {
			// Allows you to link specific lines to a runtime ID for use in certain filetypes (like entities)
			"00123456789ABCDE": "UI_THEBESTMOD"
		},
		packagedefinition: [
			{
				// For new chunks; not recommended to use
				type: "partition",
				name: "myNewChunk29",
				parent: "season3",
				partitionType: "standard"
			},
			{
				// For new bricks/entities
				type: "entity",
				partition: "myNewChunk29",
				path: "[assembly:/_pro/myBricks/myNewChunk29Map.entity].entitytemplate"
			}
		],
		thumbs: ["ConsoleCmd AAAAAAAAAA"], // Thumbs.dat commands to place after [Hitman5]
		dependencies: [
			// Runtime IDs of files to extract the dependencies of and place in chunk0 (automatic porting of dependencies) OR objects containing a runtime ID and the chunk to place the dependencies in
			"00123456789ABCDE",
			{
				runtimeID: "00AAAAAAAAAAAAAA",
				toChunk: 1, // Which chunk to place the dependencies in; optional, defaults to 0
				portFromChunk1: true // Whether to also port dependencies from chunk1 (which is usually not necessary); optional, defaults to false
			}
		],
		supportedPlatforms: [
			// Supported platforms for the mod - steam, epic and microsoft are the only currently supported platforms - if this is omitted it is assumed that all are supported
			"steam"
		],
		requirements: [
			// Required mods (if a requirement is missing the framework will not deploy and will warn the user)
			"Atampy26.RequiredMod",

			// Specify a version range for the mod using the semver range syntax (Google it) - this works for everything that takes a mod ID
			["Atampy26.OtherRequiredMod", ">2.5.0 || 1.0.2 - 2.0.1"]
		],
		incompatibilities: [
			// Incompatible mods (if present, the deploy will fail)
			"SomeOtherDeveloper.SomeOtherMod"
		],
		loadBefore: [
			// Mods that this mod should load *before* if they are enabled, i.e. mods that should override this one (the GUI will automatically sort based on this before deploying)
			"Atampy26.OtherModThatUsesThisMod"
		],
		loadAfter: [
			// Mods that this mod should load *after* if they are enabled, i.e. mods that this one should override (the GUI will automatically sort based on this before deploying)
			"Atampy26.RequiredMod", // You'll generally want to load after a required mod
			"Atampy26.OtherModThatThisModUses"
		],
		peacockPlugins: ["VeryCoolPlugin.plugin.js"], // Relative paths to plugins that Peacock should load
		scripts: ["mod.ts", "helpers.ts", "blabla.ts"], // Relative paths to TypeScript files that can alter deployment of the mod - see Scripts for more information

		options: [
			// Settings for the mod that can be enabled/disabled in the GUI - Can be omitted if the mod doesn't need to provide settings
			{
				name: "Use additional content", // Must be unique for checkbox type options
				tooltip: "Some additional content to use", // Not required, will display a tooltip to the right of the input when the user hovers over it in the GUI
				image: "additionalcontent.png", // Not required, will display an image to the right of the settings window with the image, option name and tooltip when hovered in the GUI
				type: "checkbox", // Checkbox type means a checkbox to use the variation or not
				enabledByDefault: false, // Default value of the mod option when the user first enables the mod - if it is enabled by default but requires a non-present mod it will be forcibly disabled
				contentFolders: ["additional content"], // Options can include all fields in the above section, and do not override them (so both content folders are used)
				requirements: ["Atampy26.AnotherRequiredModButOnlyIfVariationEnabled"]
			},
			{
				name: "Lowercase text",
				tooltip: "quiet text",
				type: "select", // Select type means a select box (labelled with whatever group is set as); only one option of the group can be selected and used
				group: "Use lowercase or uppercase text",
				// enabledByDefault can be omitted - its default value is false
				localisation: {
					english: {
						UI_SOMELOCALISATIONTHATUSESOPTIONS: "Some localisation that uses options"
					},
					french: {
						UI_SOMELOCALISATIONTHATUSESOPTIONS: "my french knowledge is lacking"
					},
					italian: {},
					german: {},
					spanish: {},
					russian: {},
					chineseSimplified: {},
					chineseTraditional: {},
					japanese: {}
				},
				requirements: [
					"Atampy26.ReadableUI" // As an example, lowercase text will only work with Readable UI installed because by default the game UI is capitalised
				]
			},
			{
				name: "Uppercase text",
				tooltip: "L O U D text",
				type: "select",
				group: "Use lowercase or uppercase text",
				enabledByDefault: true,
				localisation: {
					english: {
						UI_SOMELOCALISATIONTHATUSESOPTIONS: "SOME LOCALISATION THAT USES OPTIONS"
					},
					french: {
						UI_SOMELOCALISATIONTHATUSESOPTIONS: "MY FRENCH KNOWLEDGE IS LACKING"
					},
					italian: {},
					german: {},
					spanish: {},
					russian: {},
					chineseSimplified: {},
					chineseTraditional: {},
					japanese: {}
				}
			},
			{
				name: "Use epic content",
				type: "conditional", // This variation will be active if its condition returns true and inactive otherwise - it will not be shown in the GUI
				condition: '"Atampy26.SomeOtherMod" in config.loadOrder', // The condition is passed the framework's config; you can check the syntax at https://github.com/m93a/filtrex#expressions
				contentFolders: ["epicContent"]
			}
		]
	}
</script>

<div
	class="w-full h-full flex flex-col p-4 overflow-y-auto"
	use:help={{
		title: "Project hub",
		description: "The Project editor helps you configure ."
	}}
>
	<div>
		<Tabs autoWidth>
			<Tab label="Settings" />
			<Tab label="Integration" />
			<Tab label="Options" />
			<svelte:fragment slot="content">
				<TabContent>
					<FluidForm>
						<!-- Version -->
						<TextInput bind:value={config.version} labelText="Version" />

						<!-- ID -->
						<TextInput bind:value={config.id} labelText="ID" />

						<!-- Name -->
						<TextInput bind:value={config.name} labelText="Name" />

						<!-- Description -->
						<TextArea bind:value={config.description} labelText="Description" />

						<!-- Authors -->
						<TextInput bind:value={config.authors[0]} labelText="Author(s)" />
					</FluidForm>
				</TabContent>
				<TabContent>Integration</TabContent>
				<TabContent>
					<!-- Options -->
					<section>
						{#each config.options as option, index}
							<div>
								<ExpandableTile class="m-2" tileExpandedLabel="Hide variables" tileCollapsedLabel="View variables">
									<div slot="above">
										<div class="flex">
											<p class="my-2 font-semibold">
												{option.type.charAt(0).toUpperCase() + option.type.slice(1)}
												{option.group ? `(${option.group})` : ""}
											</p>
											<div class="grow"></div>
											<OverflowMenu
												flipped
												on:click={(e) => {
													e.stopPropagation()
												}}
											>
												<OverflowMenuItem
													text="Duplicate option"
													on:click={async (e) => {
														e.stopPropagation()
													}}
												/>
												<OverflowMenuItem
													danger
													text="Remove option"
													on:click={async (e) => {
														e.stopPropagation()
													}}
												/>
											</OverflowMenu>
										</div>
										{#if option.type === "checkbox"}
											<Form>
												<TextInput inline bind:value={option.name} labelText="Name" />
												<TextInput inline bind:value={option.tooltip} labelText="Tooltip" />
												<TextInput inline bind:value={option.image} labelText="Image (optional)" />
												<Checkbox class="pt-4" bind:value={option.enabledByDefault} labelText="Enable by default" />
											</Form>
										{:else if option.type === "select"}
											<Form>
												<TextInput inline bind:value={option.name} labelText="Name" />
												<TextInput inline bind:value={option.tooltip} labelText="Tooltip" />
												<TextInput inline bind:value={option.group} labelText="Group" />
												<Checkbox class="pt-4" bind:value={option.enabledByDefault} labelText="Enable by default" />
											</Form>
										{:else if option.type === "conditional"}
											<Form>
												<TextInput inline bind:value={option.name} labelText="Name" />
												<TextInput inline bind:value={option.condition} labelText="Condition" />
											</Form>
										{/if}
									</div>

									<div slot="below" class="py-8">
										<Form>
											{#if option.contentFolders}
												<TextInput inline value={option.contentFolders.join(", ")} labelText="Additional content folders" />
											{/if}
										</Form>
									</div>
								</ExpandableTile>
							</div>
						{/each}
					</section>
				</TabContent>
			</svelte:fragment>
		</Tabs>
	</div>
</div>
