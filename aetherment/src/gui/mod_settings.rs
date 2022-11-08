use crate::api::penumbra::*;

pub fn draw(state: &mut crate::Data, id_str: &str) {
	if !id_str.starts_with("aetherment-") {return}
	let Ok(id) = id_str[11..].parse::<i32>() else {return};
	let m = state.config_manager.get_mod(id);
	if m.penumbra.is_none() {m.penumbra = Some(Default::default())}
	m.mark_for_changes();
	let penumbra = m.penumbra.as_mut().unwrap();
	let collection = &mut match penumbra.iter_mut().find(|(c, _)| c == active_collection()) {
		Some(v) => v,
		None => {
			imgui::text("if i dont put this here it somehow panics without being able to catch it, idfk why\nif you are seeing this, it means this failed and your console is being spammed");
			penumbra.push((active_collection().to_owned(), crate::apply::penumbra::Config::default()));
			penumbra.last_mut().unwrap()
		}
	}.1;
	
	_ = collection.load_optionals(&root_path().join(id_str));
	let Some(options) = &collection.options else {return};
	
	imgui::dummy([0.0, 10.0]); // h * globalscale, but havent bothered bringing that over. TODO: fix
	for option in options.iter() {
		let id = option.unique();
		if !collection.settings.contains_key(id) {
			collection.settings.insert(id.to_owned(), option.default());
		}
		
		collection.settings.get_mut(id).unwrap().draw(option);
	}
	
	if m.save().unwrap_or(false) {
		// TODO: apply here
	}
}