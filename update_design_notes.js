const fs = require('fs');
const YAML = require('yaml');

const table_row_header = `| Name | Tier | Slot | Description | Mana |
| ---- | ---- | ---- | ----------- | ---- |`;

const table_of_contents = `## Table of Contents

- [Fire](#fire) (Pyromancy)
- [Lightning](#lightning) (Fulgomancy)
- [Water](#water) (Hydromancy)
- [Earth](#earth) (Geomancy)
- [Air](#air) (Aeromancy)
- [Ice](#ice) (Cryomancy)
- [Force](#force) (Trudomancy)
- [Light](#light) (Photomancy)
- [Dark](#dark) (Umbramancy)
- [Arcane](#arcane) (Arcanomancy)
- [Life](#life) (Vitomancy)
- [Death](#death) (Mortomancy)
- [Enhancement](#enhancement) (Ampiliomancy)
- [Reduction](#debuff) (Diminiomancy)
- [Summoning](#summoning) (Citomancy)
- [Necromancy](#necromancy) (Necromancy)
- [Polymorph](#polymorph) (Mutatiomancy)
- [Time](#time) (Chronomancy)`;

function spell_to_table_row(spell) {
  return `| ${spell.name} | ${spell.spell_tier} | ${spell.cast_slot} | ${spell.description} | ${spell.mana_cost} |`;
}

function string_with_space_to_title_case(str) {
  return str.split(' ').map(word => word[0].toUpperCase() + word.slice(1)).join(' ');
}

let out_str = `# Spell Idea List\n\n${table_of_contents}\n`;
const spell_data = [];

// Read in all the spell data files, parse them, and add them to the spell_data array
fs.readdirSync('./game_data/spells').forEach(file => {
  const spell = YAML.parse(fs.readFileSync(`./game_data/spells/${file}`, 'utf8'));
  spell_data.push(spell);
});

// Uniq the spell types and iterate through each one
[...new Set(spell_data.map(spell => spell.spell_type || ""))].forEach(spell_type => {
  // For each spell type, make a header and a table of the spells of that type
  out_str += `\n## ${string_with_space_to_title_case(spell_type)}\n\n`;
  out_str += table_row_header + '\n';
  // Sort the spells by tier and then by name
  for (spell of spell_data.filter(spell => spell.spell_type == spell_type).sort((a, b) => a.name.localeCompare(b.name)).sort((a, b) => a.spell_tier - b.spell_tier)) {
    out_str += spell_to_table_row(spell) + '\n';
  }
});

fs.writeFileSync('./design_notes/spells.md', out_str);
