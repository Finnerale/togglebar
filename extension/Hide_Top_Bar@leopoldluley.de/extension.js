const Main = imports.ui.main;

const Panel = Main.layoutManager.panelBox.get_child_at_index(0);

function init() {
}

function enable() {
	Panel.hide();
}

function disable() {
	Panel.show();
}

