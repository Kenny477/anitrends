<script>
    export let bindings;

    console.log('script')

    let data;
    console.log(typeof bindings.query)
    let query = bindings.query();
    query.then(res => {
        console.log(res)
    })
    console.log(query)
    bindings.query().then((res) => {
        console.log(res);
        data = res;
    });
</script>

<main>
    <div class="navbar">
        <div>
            <a class="btn btn-ghost" href="/">anitrends</a>
            {data}
        </div>
    </div>
</main>
