| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 3.1 ± 0.1 | 2.9 | 3.8 | 1.72 ± 0.28 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.8 ± 0.3 | 1.6 | 8.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.7 ± 0.1 | 2.5 | 3.7 | 1.54 ± 0.46 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.8 ± 0.5 | 1.5 | 11.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.7 ± 1.2 | 2.4 | 40.8 | 1.60 ± 0.74 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.7 ± 0.2 | 1.5 | 3.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 151.5 ± 3.0 | 143.5 | 166.5 | 1.03 ± 0.03 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 146.6 ± 3.7 | 136.9 | 177.0 | 1.00 |

