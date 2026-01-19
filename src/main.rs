use std::process::Command;
use std::os::unix::process::CommandExt;
use fltk::{app, button::Button, group, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut win = Window::new(0,0,800,600,"Docker Runner");
    app::set_foreground_color(229, 233, 240);
    app::set_background_color(46, 52, 64);
    let mut rows = group::Flex::default_fill();
    rows.set_type(group::FlexType::Column);
    let mut run = Button::default().with_label("Run");
    let mut commit = Button::default().with_label("Commit");
    rows.end();
    
    let mut run_frame: group::Flex = create_run();
    let mut commit_frame_container: group::Flex = create_commit();
    
    win.end();
    win.show();

    run_frame.hide();
    run_frame.deactivate();
    commit_frame_container.hide();
    commit_frame_container.deactivate();
    win.redraw();

    let mut value = win.clone();
    let mut value_rows = rows.clone();
    run.set_callback(move |_| {
        value_rows.hide();
        value_rows.deactivate();
        run_frame.show();
        run_frame.activate();
        value.redraw();
    });

    commit.set_callback(move |_| {
        rows.hide();
        rows.deactivate();
        commit_frame_container.show();
        commit_frame_container.activate();
        win.redraw();
    });
    
    app.run().unwrap();
    
}


fn create_run() -> group::Flex {
    let mut rows = group::Flex::default_fill();
    rows.set_type(group::FlexType::Column);
    let output = Command::new("sudo")
       .arg("docker")
       .arg("images")
       .output()
       .expect("docker images command failed");

    let images_str = match String::from_utf8(output.stdout) {
       Ok(v) => v,
       Err(e) => panic!("Invalid utf-8: {e}"),
    };
    println!("image str {images_str}");
    let images = images_str.split("\n");
    let mut image_frames: Vec<Button> = Vec::new();
    for image in images {
       image_frames.push(Button::default().with_label(image));
    }
    rows.end();

    
    for mut button in image_frames {
        button.set_callback(|btn| {
            if let Some(v) = btn.label().split_whitespace().next() {
                let _output = Command::new("sudo")
                    .arg("docker")
                    .arg("run")
                    .arg("-it")
                    .arg("--rm")
                    .arg(v)
                    .exec();
            }

        });
    }

    rows
}

fn create_commit()-> group::Flex {
    let mut rows = group::Flex::default_fill();
    rows.set_type(group::FlexType::Column);
    let output = Command::new("sudo")
       .arg("docker")
       .arg("ps")
       .output()
       .expect("docker images command failed");

    let container_str = match String::from_utf8(output.stdout) {
       Ok(v) => v,
       Err(e) => panic!("Invalid utf-8: {e}"),
    };
    println!("image str {container_str}");
    let images = container_str.split("\n");
    let mut image_frames: Vec<Button> = Vec::new();
    for image in images {
       image_frames.push(Button::default().with_label(image));
    }
    rows.end();

    
    for mut button in image_frames {
        button.set_callback(move |btn| {
            let binding = btn.label();
            let mut input = binding.split_whitespace(); 
            let container = input.next();
            let image = input.next();

            if let Some(c) = container && let Some(i) = image {
                let _output = Command::new("sudo")
                    .arg("docker")
                    .arg("commit")
                    .arg(c)
                    .arg(i)
                    .exec();
            }

        });
    }

    rows
}
