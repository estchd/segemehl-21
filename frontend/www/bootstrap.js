init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [
            {
                main,
                generate_per_file_stats,
                get_chromosome_names,
                setup_file_list,
                add_file,
                process_file,
                remove_file,
                get_file_list,
                get_dataset,
                get_file_color,
                update_file_color
            },
            {
                setup
            }
        ] = await Promise.all([
            import("/../pkg"),
            import("./index"),
        ]);
        setup(
            generate_per_file_stats,
            get_chromosome_names,
            setup_file_list,
            add_file,
            process_file,
            remove_file,
            get_file_list,
            get_dataset,
            get_file_color,
            update_file_color
        );
        main();
    } else {
        const [
            {
                default: init,
                main,
                generate_per_file_stats,
                get_chromosome_names,
                setup_file_list,
                add_file,
                process_file,
                remove_file,
                get_file_list,
                get_dataset,
                get_file_color,
                update_file_color
            },
            {
                setup
            }
        ] = await Promise.all([
            import("/../pkg"),
            import("./index"),
        ]);
        await init();
        setup(
            generate_per_file_stats,
            get_chromosome_names,
            setup_file_list,
            add_file,
            process_file,
            remove_file,
            get_file_list,
            get_dataset,
            get_file_color,
            update_file_color
        );
        main();
    }
}