nepnep_sources = [
  'main.c'
]

nepnep_deps = [
  dependency('libadwaita-1'),
  dependency('gtk4-wayland')
]

gnome = import('gnome')

executable('nepnep', nepnep_sources, dependencies: nepnep_deps, install: true)