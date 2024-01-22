const fs = require('fs');
const YAML = require('yaml');

const ByMagicSchool = {
  table_row_header: `| Name | Tier | Slot | Description | Mana |
  | ---- | ---- | ---- | ----------- | ---- |`,
  table_of_contents: `## Table of Contents

  | Spell Type | Governing Class |
  | ---------- | --------------- |
  | [Fire](#fire) | Pyromancy |
  | [Lightning](#lightning) | Fulgomancy |
  | [Water](#water) | Hydromancy |
  | [Earth](#earth) | Geomancy |
  | [Air](#air) | Aeromancy |
  | [Ice](#ice) | Cryomancy |
  | [Force](#force) | Trudomancy |
  | [Light](#light) | Photomancy |
  | [Dark](#dark) | Umbramancy |
  | [Arcane](#arcane) | Arcanomancy |
  | [Life](#life) | Vitomancy |
  | [Death](#death) | Mortomancy |
  | [Enhancement](#enhancement) | Ampiliomancy |
  | [Reduction](#reduction) | Diminiomancy |
  | [Summoning](#summoning) | Citomancy |
  | [Necromancy](#necromancy) | Necromancy |
  | [Polymorph](#polymorph) | Mutatiomancy |
  | [Time](#time) | Chronomancy |`,
  spell_to_table_row: function (spell) {
    let spell_data = spell.data;
    return `| ${spell_data.name} | ${spell_data.spellTier} | ${spell_data.castSlot} | ${spell_data.description} | ${spell_data.manaCost} |`;
  }
}

const ByTier = {
  table_row_header: `| Name | Type | Slot | Description | Mana |
  | ---- | ---- | ---- | ----------- | ---- |`,
  table_of_contents: `## Table of Contents

  | Spell Tier | Total Spells |
  | ---------- | ------------ |
  | [Tier 0](#tier-0) | #TLT0# |
  | [Tier 1](#tier-1) | #TLT1# |
  | [Tier 2](#tier-2) | #TLT2# |
  | [Tier 3](#tier-3) | #TLT3# |
  | [Tier 4](#tier-4) | #TLT4# |
  | [Tier 5](#tier-5) | #TLT5# |
  | [Tier 6](#tier-6) | #TLT6# |
  | [Tier 7](#tier-7) | #TLT7# |
  | [Tier 8](#tier-8) | #TLT8# |
  | [Tier 9](#tier-9) | #TLT9# |`,
  spell_to_table_row: function (spell) {
    let spell_data = spell.data;
    return `| ${spell_data.name} | ${spell_data.magic} | ${spell_data.castSlot} | ${spell_data.description} | ${spell_data.manaCost} |`;
  }
}

const ByManaCost = {
  table_row_header: `| Name | Tier | Slot | Description | Mana |
  | ---- | ---- | ---- | ----------- | ---- |`,
  table_of_contents: `## Table of Contents

  | Mana Cost | Total Spells |
  | --------- | ------------ |
  | [0 Mana](#0-mana) | #MC0# |
  | [1 Mana](#1-mana) | #MC1# |
  | [2 Mana](#2-mana) | #MC2# |
  | [3 Mana](#3-mana) | #MC3# |
  | [4 Mana](#4-mana) | #MC4# |`,
  spell_to_table_row: function (spell) {
    let spell_data = spell.data;
    return `| ${spell_data.name} | ${spell_data.spellTier} | ${spell_data.castSlot} | ${spell_data.description} | ${spell_data.manaCost} |`;
  }
};

function string_with_space_to_title_case(str) {
  return str.split(' ').map(word => word[0].toUpperCase() + word.slice(1)).join(' ');
}

let out_str_by_magic = `# Spell Idea List (by Magic School)\n\n${ByMagicSchool.table_of_contents}\n`;
let out_str_by_tier = `# Spell Idea List (by Tier)\n\n${ByTier.table_of_contents}\n`;
let out_str_by_mana = `# Spell Idea List (by Mana Cost)\n\n${ByManaCost.table_of_contents}\n`;
const spell_data = [];

// Read in all the spell data files, parse them, and add them to the spell_data array
fs.readdirSync('./game_data/spells').forEach(file => {
  const spell = YAML.parse(fs.readFileSync(`./game_data/spells/${file}`, 'utf8'));
  spell_data.push(spell);
});

// #### BY MAGIC ####
// Uniq the spell types and iterate through each one
[...new Set(spell_data.map(spell => spell.data.magic || ""))].forEach(magic => {
  // For each spell type, make a header and a table of the spells of that type
  out_str_by_magic += `\n## ${string_with_space_to_title_case(magic)}\n\n`;
  out_str_by_magic += ByMagicSchool.table_row_header + '\n';
  // Sort the spells by tier and then by name
  for (spell of spell_data.filter(spell => spell.data.magic == magic).sort((a, b) => a.data.name.localeCompare(b.data.name)).sort((a, b) => a.data.spellTier - b.data.spellTier)) {
    out_str_by_magic += ByMagicSchool.spell_to_table_row(spell) + '\n';
  }
});

fs.writeFileSync('./design_notes/spells.md', out_str_by_magic);

// #### BY TIER ####
// Uniq the spell tiers and iterate through each one
[...new Set(spell_data.map(spell => spell.data.spellTier || ""))].forEach(tier => {
  if (tier == "") tier = "0";
  // For each spell tier, make a header and a table of the spells of that tier
  out_str_by_tier += `\n## Tier ${tier}\n\n`;
  out_str_by_tier += ByTier.table_row_header + '\n';
  let total_in_tier = 0;
  // Sort the spells by magic type and then by name
  for (spell of spell_data.filter(spell => spell.data.spellTier == tier).sort((a, b) => a.data.name.localeCompare(b.data.name)).sort((a, b) => a.data.magic.localeCompare(b.data.magic))) {
    out_str_by_tier += ByTier.spell_to_table_row(spell) + '\n';
    total_in_tier++;
  }
  // Replace the placeholder with the total number of spells in the tier
  out_str_by_tier = out_str_by_tier.replace(`#TLT${tier}#`, total_in_tier);
});

fs.writeFileSync('./design_notes/spells_by_tier.md', out_str_by_tier);

// #### BY MANA COST ####
// Uniq the mana costs and iterate through each one
[...new Set(spell_data.map(spell => spell.data.manaCost || ""))].forEach(mana => {
  if (mana == "") mana = "0";
  // For each mana cost, make a header and a table of the spells of that mana cost
  out_str_by_mana += `\n## ${mana} Mana\n\n`;
  out_str_by_mana += ByManaCost.table_row_header + '\n';
  let total_in_mana = 0;
  // Sort the spells by tier and then by name
  for (spell of spell_data.filter(spell => spell.data.manaCost == mana).sort((a, b) => a.data.name.localeCompare(b.data.name)).sort((a, b) => a.data.spellTier - b.data.spellTier)) {
    out_str_by_mana += ByManaCost.spell_to_table_row(spell) + '\n';
    total_in_mana++;
  }
  // Replace the placeholder with the total number of spells in the mana cost
  out_str_by_mana = out_str_by_mana.replace(`#MC${mana}#`, total_in_mana);
});

fs.writeFileSync('./design_notes/spells_by_mana_cost.md', out_str_by_mana);
