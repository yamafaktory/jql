| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.8 ± 0.1 | 2.7 | 3.8 | 1.61 ± 0.18 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.7 ± 0.2 | 1.5 | 3.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.4 ± 0.2 | 2.3 | 5.7 | 1.46 ± 0.19 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.7 ± 0.2 | 1.4 | 5.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.4 ± 0.2 | 2.3 | 7.7 | 1.45 ± 0.22 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.7 ± 0.2 | 1.5 | 3.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 132.9 ± 0.9 | 131.4 | 140.9 | 1.00 ± 0.02 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 132.5 ± 2.0 | 128.4 | 146.1 | 1.00 |

