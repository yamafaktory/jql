| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.3 ± 1.8 | 27.9 | 50.9 | 16.76 ± 2.38 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.9 ± 0.3 | 1.6 | 7.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.1 ± 1.4 | 27.8 | 37.4 | 17.66 ± 2.06 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.8 ± 0.2 | 1.3 | 3.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 31.9 ± 1.5 | 27.8 | 46.2 | 17.04 ± 2.57 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.9 ± 0.3 | 1.6 | 7.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 164.1 ± 4.7 | 144.9 | 191.8 | 1.32 ± 0.06 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 124.8 ± 4.4 | 112.5 | 144.9 | 1.00 |

