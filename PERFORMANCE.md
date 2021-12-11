| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.5 ± 0.6 | 31.5 | 34.6 | 16.46 ± 1.53 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.2 | 1.7 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.4 ± 0.6 | 31.4 | 35.6 | 17.40 ± 3.26 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.9 ± 0.3 | 1.6 | 8.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.4 ± 0.6 | 31.4 | 34.5 | 17.46 ± 1.79 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.9 ± 0.2 | 1.6 | 4.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 164.6 ± 1.1 | 163.4 | 191.1 | 1.29 ± 0.02 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 127.1 ± 1.5 | 122.8 | 139.8 | 1.00 |

