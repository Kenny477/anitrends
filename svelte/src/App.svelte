<script>
    export let bindings;

    let res = bindings.query();

    function chooseTitle(title) {
        return title.romanji
            ? title.romanji
            : title.english
            ? title.english
            : title.native
            ? title.native
            : "No title found";
    }
</script>

<main class="h-screen">
    <div class="navbar">
        <div>
            <a class="btn btn-ghost" href="/">anitrends</a>
        </div>
    </div>
    <div class="carousel space-x-4 rounded-box px-10">
        {#await res}
            <div class="radial-progress animate-spin" style="--value:70;" />
        {:then res}
            {#each res.data.Page.airingSchedules as item}
                <div class="carousel-item w-52">
                    <div class="card image-full bg-base-100 shadow-xl wrap">
                        <figure>
                            <img
                                src={item.media.coverImage.extraLarge}
                                alt={chooseTitle(item.media.title)}
                            />
                        </figure>
                        <div class="card-body">
                            <h1 class="card-title text-md">
                                {chooseTitle(item.media.title)}
                            </h1>
                            <!-- <p class="text-sm truncate">
                                {@html item.media.description
                                    ? item.media.description
                                    : ""}
                            </p> -->
                        </div>
                    </div>
                </div>
            {/each}
        {/await}
    </div>
</main>
