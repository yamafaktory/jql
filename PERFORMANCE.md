| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.4 ± 0.6 | 31.5 | 37.9 | 15.67 ± 2.62 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.1 ± 0.3 | 1.6 | 7.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.7 ± 0.8 | 31.4 | 38.3 | 14.98 ± 6.02 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.2 ± 0.9 | 1.6 | 13.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.5 ± 0.6 | 31.4 | 35.2 | 15.29 ± 2.69 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.1 ± 0.4 | 1.6 | 6.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 164.0 ± 0.9 | 162.8 | 183.1 | 1.36 ± 0.02 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 120.6 ± 1.6 | 117.5 | 137.0 | 1.00 |

