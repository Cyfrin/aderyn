## Project Overview

### Configuration
- **Root:** {{ root }}
- **Source:** {{ source }}
- **Remappings:**
{%if remappings.len() > 0 %}
{% for r in remappings %}
  - "{{ r }}"
{% endfor %}
{% else %}
  - No remappings found
{% endif %}

### Notes:
- *Root* directory is an absolute path.
- *Remappings* can be relative or absolute. The relative ones are relative to the root.
- Not all solidity files in the root directory are authored by the developer. Some files are 3rd party libraries which should be omitted for analysis unless explicitly pulled as a dependency by other developer authored solidity files.
- *Source* directory is the most important directory of all. It lives inside the *Root* and contains all the solidity contracts that the developer has worked on.

Aderyn has determined that there are **{{ compilation_units.len() }} compilation units**. The actual Solc versions itself are not mentioned.

Note that the file paths shown are relative to the project's **Root** shown above. Please also note that only a subset of these files are considered included files - those lines of entries are appended with `INCLUDED` keyword.

{% for cu in compilation_units %}
### Compilation Unit {{ loop.index }}:
{% for f in cu.files %}
- {{ f.path }}{% if f.included %} INCLUDED{% endif %}
{% endfor %}

{% endfor %}

### Summary:
- Total compilation units: **{{ compilation_units.len() }}**
{% for cu in compilation_units %}
- Compilation Unit {{ loop.index }} has **{{ cu.files.len() }}** files of which **{{ cu.included_count }}** are included.
{% endfor %}
