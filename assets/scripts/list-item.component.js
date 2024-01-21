class ListItem extends HTMLElement {
    constructor() {
        super();

        const div = document.createElement("div");

        const link = this.getAttribute("link");
        const title = this.getAttribute("title");
        const date = this.getAttribute("date");
        const intro = this.getAttribute("intro");

        if (link === null) {
            throw new Error("ListItem's link attributes is not available. Unable to render the page.");
        }
        div.innerHTML = `<div
    class="card bg-base-200 shadow-xl mb-5 lg:ml-20 lg:mr-20 rounded-lg select-none cursor-pointer hover:bg-base-300 transition-colors">
    <a
        href=${link}
        hx-get=${link}
        hx-swap="transition:true show:window:top"
        hx-target="#content"
        hx-push-url=${link}>
        <div class="card-body z-30" style="color: black">
            <div class="flex lg:flex-row flex-col gap-2">
                <h1 class="card-title grow transition-colors" style="font-size: 1.25rem; font-weight: 600; margin: 0">${
                    title !== null ? title : "No Title"
                }</h1>
                <h2 class="justify-end" style="font-size: .875rem; font-weight: normal; margin: 0">${
                    date !== null ? date : "Unknown"
                }</h2>
            </div>
            ${intro !== null ? `<p style=\"font-size: revert; margin: 0\"> ${intro} </p>` : `<div></div>`}
        </div>
    </a>
</div>
`;

        this.append(div);
    }
}

window.customElements.define("list-item", ListItem);
