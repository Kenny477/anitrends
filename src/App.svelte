<script>
	export let bindings;

	let airing = bindings.airing();
	let trending = bindings.build_trending(10);
	console.log("trending", trending);

	function chooseTitle(title) {
		return title.romanji
			? title.romanji
			: title.english
			? title.english
			: title.native
			? title.native
			: "No title found";
	}
	
	// Cache result of API calls to trending for 24 hours
	// See https://dev.to/danawoodman/svelte-quick-tip-connect-a-store-to-local-storage-4idi#:~:text=Here's%20a%20really%20quick%20tip,settings%20retained%20for%20future%20sessions.
</script>

<main class="h-screen">
	<div class="navbar">
		<div>
			<a class="btn btn-ghost" href="/">anitrends</a>
		</div>
	</div>
	{#await trending}
		<div class="radial-progress animate-spin" style="--value:70;" />
	{:then trending}
		<div class="flex flex-row space-x-4 overflow-x-scroll h-96 ">
			{#each trending.trending as item}
				<div>
					<div class="card w-64 h-[85%] bg-base-100 shadow-xl">
						<figure class="h-1/2 bg-black">
							<img
								class="object-cover h-full w-full"
								src={item.coverImage.extraLarge}
								alt={chooseTitle(item.title)}
							/>
						</figure>
						<div class="card-body">
							<h1 class="card-title text-md">
								{chooseTitle(item.title)}
							</h1>
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/await}

	{#await airing}
		<div class="radial-progress animate-spin" style="--value:70;" />
	{:then airing}
		<div class="flex flex-row space-x-4 overflow-x-scroll h-96 ">
			{#each airing.data.Page.airingSchedules as item}
				<div>
					<div class="card w-64 h-[85%] bg-base-100 shadow-xl">
						<figure class="h-1/2 bg-black">
							<img
								class="object-cover h-full w-full"
								src={item.media.coverImage.extraLarge}
								alt={chooseTitle(item.media.title)}
							/>
						</figure>
						<div class="card-body">
							<h1 class="card-title text-md">
								{chooseTitle(item.media.title)}
								<p class="text-sm opacity-50">{item.episode}</p>
							</h1>
							<p class="text-sm">
								{new Date(item.airingAt * 1000).toLocaleString(
									undefined,
									{ dateStyle: "medium", timeStyle: "short" }
								)}
							</p>
							<!-- <p class="text-sm truncate">
                                {@html item.media.description
                                    ? item.media.description
                                    : ""}
                            </p> -->
						</div>
					</div>
				</div>
			{/each}
		</div>
	{/await}
</main>
